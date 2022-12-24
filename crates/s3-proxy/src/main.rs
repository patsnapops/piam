// #![allow(unused)]

use std::net::SocketAddr;

use axum::{
    routing::{any, get, put},
    Router,
};
use busylib::logger::init_logger;
use log::info;
use patsnap_constants::policy_model::OBJECT_STORAGE;
use piam_core::{
    config::{server_port, set_constants, STATE_UPDATE_INTERVAL},
    error::eok,
    state::StateManager,
};

use crate::{
    config::{features, S3Config},
    handler::S3ProxyState,
};

mod config;
mod error;
mod handler;
mod request;
#[cfg(feature = "uni-key")]
mod uni_key;

#[tokio::main]
async fn main() {
    let bin_name = env!("CARGO_PKG_NAME").replace('-', "_");
    let (_guard, _log_handle) = init_logger(&bin_name, true);
    set_constants("[Patsnap S3 Proxy]", OBJECT_STORAGE);

    // TODO: make this async
    let state_manager = StateManager::initialize().await;
    let state: S3ProxyState = state_manager.arc_state.clone();
    // TODO: move this into state::StateManager
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(STATE_UPDATE_INTERVAL)).await;
            state_manager.update_state().await;
        }
    });

    let routes = Router::new()
        .route("/health", get(handler::health))
        .route("/_piam_manage_api", put(handler::manage))
        // the router for ListBucket only
        .route("/", any(handler::handle))
        // the router for other operations
        .route("/*path", any(handler::handle_path))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], server_port()));
    info!(
        "S3 compliant proxy listening on {} with features {}",
        addr,
        features()
    );
    eok(axum::Server::bind(&addr)
        .serve(routes.into_make_service_with_connect_info::<SocketAddr>())
        .await);
}
