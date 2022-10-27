#![allow(unused)]

use std::net::SocketAddr;

use axum::{routing::get, Router};
use log::info;
use piam_tracing::logger::init_logger;

mod config;
mod handler;
mod store;

#[tokio::main]
async fn main() {
    let bin_name = env!("CARGO_PKG_NAME").replace('-', "_");
    let (_guard, _log_handle) = init_logger(&bin_name, true);

    let app = Router::new()
        // .route("/_piam_manage_api", put(handler::manage))
        .route("/health", get(handler::health))
        .route("/principals", get(handler::get_principals))
        .route("/policies/:kind", get(handler::get_policies))
        .route(
            "/amz_sign_params/:service/:region",
            get(handler::get_amz_sign_params),
        )
        .route("/config/:service/:region", get(handler::get_config));

    let addr = SocketAddr::from(([0, 0, 0, 0], config::port()));
    info!("piam-manager listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
