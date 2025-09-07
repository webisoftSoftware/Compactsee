use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractEvent {
    #[serde(rename = "__typename")]
    pub type_name: String,
    pub state: String,
    pub address: String,
    #[serde(rename = "chainState")]
    pub chain_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    ContractEvent(ContractEvent),
    Disconnect,
    TimeLeft(u64),
}
