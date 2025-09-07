use std::time::Duration;

use crate::domain::{ContractEvent, Event};
use futures::channel::mpsc::Sender;
use futures_util::{SinkExt, StreamExt};
use leptos::{error::Error, prelude::ServerFnErrorErr};
use midnight_node_ledger_helpers::{deserialize, ContractState, NetworkId, DB};
use serde_json::json;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{self, client::IntoClientRequest, http::HeaderValue, Message},
};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

#[derive(Clone)]
pub struct ContractIndexer {
    network_id: NetworkId,
    indexer_ws: String,
    timeout: u64,
}

impl ContractIndexer {
    pub fn new(network_id: NetworkId, indexer_ws: String, timeout: u64) -> Self {
        Self {
            network_id,
            indexer_ws,
            timeout,
        }
    }

    pub async fn subscribe_to_contract<D>(
        &self,
        contract_address: String,
        mut tx: Sender<Event>,
    ) -> Result<(), Error>
    where
        D: DB + Clone + Send + Sync,
    {
        // we need to prepend the network id to the contract. if its testnet, its 02
        // might change in the future if midnight tries to address it
        let contract_addr = format!("0{}{}", self.network_id as u8, contract_address);
        info!("contract address is {}", contract_addr.clone());

        let timeout_token = CancellationToken::new();
        let timeout_token_clone = timeout_token.clone();
        let timeout = self.timeout;

        // Kill the socket connection set by timeout. We want to avoid a resource leak if the user closes browser
        // Not sure if there is a cleaner way to do this
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(timeout)).await;
            timeout_token_clone.cancel();
        });

        let mut request = self.indexer_ws.clone().into_client_request()?;
        request.headers_mut().insert(
            "Sec-WebSocket-Protocol",
            HeaderValue::from_static("graphql-ws"),
        );
        let (ws_stream, response) = connect_async(request).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        let init_message = json!({
            "type": "connection_init"
        });
        info!("Sending connection_init message");
        ws_sender
            .send(Message::Text(init_message.to_string()))
            .await?;

        let _message = ws_receiver.next().await.ok_or_else(|| {
            ServerFnErrorErr::ServerError("No response from web socket connection_init".to_string())
        })?;
        info!("Received connection_init message");

        let subscription_query = format!(
            r#"
                subscription ContractSync {{
                    contractActions(address: "{}") {{
                        __typename
                        ... on ContractDeploy {{
                            address
                            state
                            chainState
                        }}
                        ... on ContractCall {{
                            address
                            state
                            chainState
                        }}
                        ... on ContractUpdate {{
                            address
                            state
                            chainState
                        }}
                    }}
                }}
            "#,
            contract_addr
        );

        info!("Sending subscribe message");
        let start_message = json!({
            "id": "contract-sync",
            "type": "subscribe",
            "payload": {
                "query": subscription_query
            }
        });

        ws_sender
            .send(Message::Text(start_message.to_string()))
            .await?;

        let mut ping_to_midnight = 0;
        let mut total_ellapsed_time = 0;
        let ping_interval = tokio::time::interval(std::time::Duration::from_secs(1));
        tokio::pin!(ping_interval);
        loop {
            tokio::select! {
            _ = ping_interval.tick() => {
                    total_ellapsed_time += 1;
                    tx.try_send(Event::TimeLeft(timeout- total_ellapsed_time))?;
                    // Send periodic ping to keep connection alive
                    ping_to_midnight += 1;
                    if ping_to_midnight == 30 {
                        ping_to_midnight = 0;
                        info!("sending tick");
                        if let Err(e) = ws_sender.send(Message::Ping(vec![])).await {
                        error!("Failed to send ping: {}", e);
                        break;
                    }
                    }

                }

            _ = timeout_token.cancelled() => {
                    info!("Operation cancelled after timeout");
                     tx.try_send(Event::Disconnect)?;
                    break;
                }

            msg =  ws_receiver.next() => {
                info!("Received message {:#?}", msg);
                    match msg {
                        Some(Ok(tungstenite::Message::Text(text))) => {
                            match serde_json::from_str::<serde_json::Value>(&text) {
                                Ok(parsed) => {
                                    info!("parsed is {:#?}", parsed);
                                    if let Some(contract_action) = parsed
                                        .get("payload")
                                        .and_then(|p| p.get("data"))
                                        .and_then(|d| d.get("contractActions"))
                                    {
                                        match serde_json::from_value::<ContractEvent>(
                                            contract_action.clone(),
                                        ) {
                                            Ok(mut event) => {
                                                let tx_raw = hex::decode(event.state.clone())?;
                                                match deserialize::<ContractState<D>, _>(std::io::Cursor::new(tx_raw), self.network_id) {
                                                    Ok(state) => {
                                                        let state_as_string = format!("{:#?}", state.data);
                                                        info!("state is {}", state_as_string.clone());
                                                        event.state = state_as_string;
                                                    },
                                                    Err(e) => {
                                                        error!("boooo did not parse state {}", e);
                                                    },
                                                }
                                                tx.try_send(Event::ContractEvent(event))?
                                            },
                                            Err(e) => {
                                                error!("Issue mapping parsed value to ContractEvent {}", e);
                                                break;
                                            }
                                        }
                                    } else {
                                        error!("No contract data");
                                        break;

                                    }
                                }
                                Err(e) => {
                                    error!("Error parsing message: {}", e);
                                    break;

                                }
                            }
                        }
                        Some(Ok(tungstenite::Message::Pong(data))) => {
                            // Respond to ping with pong
                            info!("got pong {:#?}", data);
                        }
                        Some(Ok(tungstenite::Message::Close(_))) => {
                            info!("Received close frame from server");
                            break;
                        }
                        Some(Ok(_)) => {
                            info!("Unexpected message type");
                        }
                        Some(Err(e)) => {
                            error!("{}", e);
                        }
                        None => {
                            info!("WebSocket stream ended");
                            break;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
