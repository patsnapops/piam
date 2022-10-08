// #![allow(dead_code)]
#![allow(unused)]

use std::net::SocketAddr;

use axum::{
    routing::{any, put},
    Router,
};
use axum::routing::get;
use log::info;
use piam_core::config::{CoreConfig, PIAM_MANAGER_ADDRESS};
use piam_core::state::StateUpdater;
use piam_tracing::logger::init_logger;
use crate::config::{proxy_port, S3Config};

use crate::handler::S3ProxyState;

mod config;
mod handler;
mod parser;
mod policy;
mod request;

#[tokio::main]
async fn main() {
    let bin_name = env!("CARGO_PKG_NAME").replace('-', "_");
    let (_guard, _log_handle) = init_logger(&bin_name, true);

    let state_updater = StateUpdater::new_with_kind("ObjectStorage").await;
    let state: S3ProxyState = state_updater.state.clone();

    let app = Router::with_state(state)
        .route("/health", get(handler::health))
        .route("/_piam_manage_api", put(handler::manage))
        // the router for ListBucket only
        .route("/", any(handler::handle))
        // the router for other operations
        .route("/*path", any(handler::handle_path));

    tokio::spawn(async move {
        loop {
            S3Config::update_all().await;
            state_updater.update().await;
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        }
    });

    let addr = SocketAddr::from(([0, 0, 0, 0], proxy_port()));
    info!("s3 compliant proxy listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
