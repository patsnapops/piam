use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use log::{error, info};

use crate::{
    error::{ManagerError, ManagerResult},
    persist::get_resource_string,
};

pub async fn health() -> impl IntoResponse {
    "OK"
}

pub async fn get_accounts(Path(ver): Path<String>) -> ManagerResult<String> {
    info!("version: {} api: get_accounts", ver);
    get_resource_string("accounts").await
}

pub async fn get_users(Path(ver): Path<String>) -> ManagerResult<String> {
    info!("version: {} api: get_users", ver);
    get_resource_string("users").await
}

pub async fn get_groups(Path(ver): Path<String>) -> ManagerResult<String> {
    info!("version: {} api: get_groups", ver);
    get_resource_string("groups").await
}

pub async fn get_policies(
    Path((ver, policy_model)): Path<(String, String)>,
) -> ManagerResult<String> {
    info!(
        "version: {} api: get_policies policy_model: {}",
        ver, policy_model
    );
    get_resource_string(format!("policies:{}", policy_model).as_str()).await
}

pub async fn get_user_group_relationships(Path(ver): Path<String>) -> ManagerResult<String> {
    info!("version: {} api: get_user_group_relationships", ver);
    get_resource_string("user_group_relationships").await
}

pub async fn get_policy_relationships(Path(ver): Path<String>) -> ManagerResult<String> {
    info!("version: {} api: get_policy_relationships", ver);
    get_resource_string("policy_relationships").await
}

pub async fn extended_config(
    Path((ver, config_type)): Path<(String, String)>,
) -> ManagerResult<String> {
    info!("version: {} api: extended_config: {}", ver, config_type);
    get_resource_string(format!("extended_config:{}", config_type).as_str()).await
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
