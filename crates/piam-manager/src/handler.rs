use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use log::{error, info};
use piam_core::crypto::encrypt;

use crate::{
    error::{ManagerError, ManagerResult},
    persist::get_resource_string,
};

pub async fn health() -> impl IntoResponse {
    "OK"
}

pub async fn get_accounts(Path(ver): Path<String>) -> ManagerResult<String> {
    info!("version: {} api: get_accounts", ver);
    let r = get_resource_string("accounts").await?;
    wrap(r)
}

pub async fn get_users(Path(ver): Path<String>) -> ManagerResult<String> {
    info!("version: {} api: get_users", ver);
    let r = get_resource_string("users").await?;
    wrap(r)
}

pub async fn get_groups(Path(ver): Path<String>) -> ManagerResult<String> {
    info!("version: {} api: get_groups", ver);
    let r = get_resource_string("groups").await?;
    wrap(r)
}

pub async fn get_policies(
    Path((ver, policy_model)): Path<(String, String)>,
) -> ManagerResult<String> {
    info!(
        "version: {} api: get_policies policy_model: {}",
        ver, policy_model
    );
    let r = get_resource_string(&format!("policies:{}", policy_model)).await?;
    wrap(r)
}

pub async fn get_user_group_relationships(Path(ver): Path<String>) -> ManagerResult<String> {
    info!("version: {} api: get_user_group_relationships", ver);
    let r = get_resource_string("user_group_relationships").await?;
    wrap(r)
}

pub async fn get_policy_relationships(Path(ver): Path<String>) -> ManagerResult<String> {
    info!("version: {} api: get_policy_relationships", ver);
    let r = get_resource_string("policy_relationships").await?;
    wrap(r)
}

pub async fn extended_config(
    Path((ver, config_type)): Path<(String, String)>,
) -> ManagerResult<String> {
    info!("version: {} api: extended_config: {}", ver, config_type);
    let r = get_resource_string(&format!("extended_config:{}", config_type)).await?;
    wrap(r)
}

fn wrap(value: String) -> ManagerResult<String> {
    // manually encrypt for HTTP
    Ok(encrypt(value))
}

impl IntoResponse for ManagerError {
    fn into_response(self) -> Response {
        let body = match self {
            ManagerError::BadRequest(e) => e,
            ManagerError::Internal(e) => e,
        };
        error!("ManagerError: {}", body);
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

#[cfg(test)]
mod tests {}
