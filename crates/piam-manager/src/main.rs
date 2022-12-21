// #![allow(unused)]

use std::net::SocketAddr;

use axum::{routing::get, Router};
use log::info;
use piam_common::{
    logger::init_logger,
    manager_api::{
        ACCOUNTS, CONFIG_TYPE, EXTENDED_CONFIG, GROUPS, POLICIES, POLICY_MODEL,
        POLICY_RELATIONSHIPS, USERS, USER_GROUP_RELATIONSHIPS,
    },
};
use piam_proxy_core::error::eok;

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
        .route(&gen_path(ACCOUNTS), get(handler::get_accounts))
        .route(&gen_path(USERS), get(handler::get_users))
        .route(&gen_path(GROUPS), get(handler::get_groups))
        .route(
            &gen_path_with_param(POLICIES, POLICY_MODEL),
            get(handler::get_policies),
        )
        .route(
            &gen_path(USER_GROUP_RELATIONSHIPS),
            get(handler::get_user_group_relationships),
        )
        .route(
            &gen_path(POLICY_RELATIONSHIPS),
            get(handler::get_policy_relationships),
        )
        .route(
            &gen_path_with_param(EXTENDED_CONFIG, CONFIG_TYPE),
            get(handler::extended_config),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], config::port()));
    info!("piam-manager listening on {}", addr);
    eok(axum::Server::bind(&addr)
        .serve(routes.into_make_service_with_connect_info::<SocketAddr>())
        .await);
}

fn gen_path(value: &str) -> String {
    format!("/:v/{}", value)
}

fn gen_path_with_param(value: &str, param: &str) -> String {
    format!("/:v/{}/:{}", value, param)
}
