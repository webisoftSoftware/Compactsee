use crate::core::contract_indexer::ContractIndexer;
use axum::extract::FromRef;
use leptos::config::LeptosOptions;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub contract_indexer: ContractIndexer,
}
