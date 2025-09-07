#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use compactsee::app::*;
    use compactsee::core::{app_state::AppState, contract_indexer::ContractIndexer};
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use midnight_node_ledger_helpers::NetworkId;

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("midnight_node_ledger_helpers=info".parse().unwrap())
                .add_directive("midnight_ledger_prototype=info".parse().unwrap())
                .add_directive(tracing::Level::INFO.into()),
        )
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(true)
        .with_timer(tracing_subscriber::fmt::time::time())
        .init();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let indexer_ws = "wss://indexer.testnet-02.midnight.network/api/v1/graphql/ws".to_string();
    let network_id = NetworkId::TestNet; // hard coding to testnet for now. Make this configurable later
    let contract_indexer = ContractIndexer::new(network_id, indexer_ws, 300);

    // set up app state
    let app_state = AppState {
        contract_indexer,
        leptos_options,
    };

    let app = Router::new()
        .leptos_routes_with_context(
            &app_state,
            routes,
            {
                let app_state = app_state.clone();
                move || provide_context(app_state.clone())
            },
            {
                let leptos_options = app_state.leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
