use crate::{
    domain::{ContractEvent, Event},
    ui::components::contract_panel::ContractPanel,
};
use leptos::{prelude::*, task::spawn_local};
use server_fn::{codec::JsonEncoding, BoxedStream, ServerFnError, Websocket};

#[component]
pub fn HomePage() -> impl IntoView {
    use futures::{channel::mpsc, StreamExt};
    let contract_address = RwSignal::new("".to_string());
    let (mut tx, rx) = mpsc::channel::<String>(100);
    let (connected, is_connected) = signal(false);
    let (show_progress_bar, set_show_progress_bar) = signal(false);
    let (error_message, set_error_message) = signal(None::<String>);
    let (timeleft, set_timeleft) = signal(None::<u64>);
    let (show_about, set_show_about) = signal(true);

    let (contract_events, set_contract_events) = signal(Vec::<ContractEvent>::new());

    if cfg!(feature = "hydrate") {
        spawn_local(async move {
            match connect_to_contract(rx.map(|s| Ok(s)).into()).await {
                Ok(mut messages) => {
                    while let Some(msg) = messages.next().await {
                        match msg {
                            Ok(event) => match event {
                                Event::ContractEvent(contract_event) => {
                                    is_connected.set(true);
                                    set_show_progress_bar.set(false);
                                    set_contract_events.update(|contract_events| {
                                        contract_events.push(contract_event);
                                    });
                                }
                                Event::Disconnect => {
                                    // figure out this later. We want to somehow keep the connection alive after it is disconnected
                                    // is_connected.set(false);
                                }
                                Event::TimeLeft(timeleft) => {
                                    set_timeleft.set(Some(timeleft));
                                }
                            },
                            Err(e) => {
                                is_connected.set(true); // we want to disable this since most likely the websocket is closed. Force the refresh
                                set_show_progress_bar.set(false);
                                set_error_message.set(Some(format!(
                                    "Refresh your current web page and reconnect. \n {}.",
                                    e
                                )))
                            }
                        }
                    }
                }
                Err(e) => {
                    set_show_progress_bar.set(false);
                    set_error_message.set(Some(format!("Error connecting to contract: {}", e)))
                }
            }
        });
    }

    let contract_connect = move |_| {
        set_show_progress_bar.set(true);
        set_error_message.set(None);
        set_show_about.set(false);

        match tx.try_send(contract_address.get()) {
            Ok(_) => {}
            Err(e) => {
                set_show_progress_bar.set(false);
                set_error_message.set(Some(format!("Error sending contract address: {}", e)))
            }
        }
    };

    view! {
        <div class="flex flex-col items-center justify-center gap-4 mt-8">
            <div class="flex flex-row gap-2">
                <input type="text" class="input w-80 md:w-96" disabled=connected placeholder="Enter Contract Address" bind:value=contract_address />
                <button on:click=contract_connect disabled=connected class="btn btn-neutral">"Connect"</button>
            </div>

            <Show
                when=move || { show_about.get() }
                fallback=move || view! {}>
                <blockquote class="alert not-italic items-start text-xs leading-loose *:m-0!">
                    <p><svg class="size-4 ms-2 inline-block text-info" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g fill="currentColor" stroke-linejoin="miter" stroke-linecap="butt"><circle cx="12" cy="12" r="10" fill="none" stroke="currentColor" stroke-linecap="square" stroke-miterlimit="10" stroke-width="2"></circle><path d="m12,17v-5.5c0-.276-.224-.5-.5-.5h-1.5" fill="none" stroke="currentColor" stroke-linecap="square" stroke-miterlimit="10" stroke-width="2"></path><circle cx="12" cy="7.25" r="1.25" fill="currentColor" stroke-width="2"></circle></g></svg></p>
                    <p>"👋 Enter your testnet contract address and we will listen for any contract events for 5 minutes."</p>
                </blockquote>

            </Show>


            <Show
                when=move || { error_message.get().is_some() }
                fallback=move || view! {}>
                <div role="alert" class="alert alert-error alert-soft">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                    <span class="whitespace-pre-line">{error_message.get().unwrap_or("Something went wrong".to_string())}</span>
                </div>
            </Show>


            <Show
                when=move || { timeleft.get().is_some() && connected.get() }
                fallback=move || view! {}>
                <div class="p-2">
                    <div class="badge badge-soft badge-warning">Time left: {timeleft.get().unwrap()}</div>
                </div>
            </Show>

            <div class="max-w-4xl mx-auto" class:hidden=move || !connected.get() >
                <ContractPanel contract_events=contract_events />
            </div>

            <div class="flex w-52 flex-col gap-4" class:hidden=move || !show_progress_bar.get()>
                <div class="skeleton h-32 w-full"></div>
                <div class="skeleton h-4 w-28"></div>
                <div class="skeleton h-4 w-full"></div>
                <div class="skeleton h-4 w-full"></div>
            </div>
        </div>
    }
}

#[server(protocol = Websocket<JsonEncoding, JsonEncoding>)]
pub async fn connect_to_contract(
    input: BoxedStream<String, ServerFnError>,
) -> Result<BoxedStream<Event, ServerFnError>, ServerFnError> {
    use crate::core::app_state::AppState;
    use futures::{channel::mpsc, StreamExt};
    use midnight_node_ledger_helpers::DefaultDB;
    use tracing::info;

    let mut input = input;
    let (tx, rx) = mpsc::channel(100);
    let app_state = use_context::<AppState>().ok_or::<ServerFnError>(
        ServerFnError::ServerError("Could not extract app state".to_string()),
    )?;

    tokio::spawn(async move {
        if let Some(Ok(contract_address)) = input.next().await {
            info!("got contract address");
            let _ = app_state
                .contract_indexer
                .subscribe_to_contract::<DefaultDB>(contract_address, tx)
                .await;
        }
    });

    Ok(rx.map(|event| Ok(event)).into())
}
