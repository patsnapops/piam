#![allow(unused)]

extern crate core;

use std::{net::SocketAddr, sync::Arc};

use axum::{
    routing::{any, get, put},
    Router,
};
use log::{debug, info};
use piam_proxy_core::{
    config::{proxy_port, set_policy_model, set_proxy_type, STATE_UPDATE_INTERVAL},
    state::StateManager,
};
use piam_tracing::logger::init_logger;

use crate::{
    config::{features, S3Config},
    handler::S3ProxyState,
};

mod config;
mod error;
mod handler;
mod parser;
mod policy;
mod request;
#[cfg(feature = "uni-key")]
mod uni_key;

#[tokio::main]
async fn main() {
    set_proxy_type("[Patsnap S3 Proxy]");
    set_policy_model("ObjectStorage");
    let bin_name = env!("CARGO_PKG_NAME").replace('-', "_");
    let (_guard, _log_handle) = init_logger(&bin_name, true);

    let state_manager = StateManager::initialize().await;
    let state: S3ProxyState = state_manager.arc_state.clone();
    // TODO: move this into state::StateManager
    tokio::spawn(async move {
        loop {
            state_manager.update_state().await;
            tokio::time::sleep(std::time::Duration::from_secs(STATE_UPDATE_INTERVAL)).await;
        }
    });

    let app = Router::with_state(state)
        .route("/health", get(handler::health))
        .route("/_piam_manage_api", put(handler::manage))
        // the router for ListBucket only
        .route("/", any(handler::handle))
        // the router for other operations
        .route("/*path", any(handler::handle_path));

    let addr = SocketAddr::from(([0, 0, 0, 0], proxy_port()));
    info!(
        "S3 compliant proxy listening on {} with features {}",
        addr,
        features()
    );
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Server error");
}
