// #![allow(unused)]

use std::net::SocketAddr;

use axum::{routing::get, Router};
use log::info;
use piam_tracing::logger::init_logger;

mod config;
mod error;
mod handler;
mod persist;

#[tokio::main]
async fn main() {
    let bin_name = env!("CARGO_PKG_NAME").replace('-', "_");
    let (_guard, _log_handle) = init_logger(&bin_name, true);

    let routes = Router::new()
        // .route("/_piam_manage_api", put(handler::manage))
        .route("/health", get(handler::health))
        .route("/accounts", get(handler::get_accounts))
        .route("/users", get(handler::get_users))
        .route("/groups", get(handler::get_groups))
        .route("/policies/:policy_model", get(handler::get_policies))
        .route(
            "/user_group_relationships",
            get(handler::get_user_group_relationships),
        )
        .route(
            "/policy_relationships",
            get(handler::get_policy_relationships),
        )
        .route(
            "/extended_config/:config_type",
            get(handler::extended_config),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], config::port()));
    info!("piam-manager listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(routes.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
