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

pub async fn get_accounts() -> ManagerResult<String> {
    info!("get_accounts");
    get_resource_string("accounts").await
}

pub async fn get_users() -> ManagerResult<String> {
    info!("get_users");
    get_resource_string("users").await
}

pub async fn get_groups() -> ManagerResult<String> {
    info!("get_groups");
    get_resource_string("groups").await
}

pub async fn get_policies(Path(policy_model): Path<String>) -> ManagerResult<String> {
    info!("get_policies: {}", policy_model);
    get_resource_string(format!("policies:{}", policy_model).as_str()).await
}

pub async fn get_user_group_relationships() -> ManagerResult<String> {
    info!("get_user_group_relationships");
    get_resource_string("user_group_relationships").await
}

pub async fn get_policy_relationships() -> ManagerResult<String> {
    info!("get_policy_relationships");
    get_resource_string("policy_relationships").await
}

pub async fn extended_config(Path(config_type): Path<String>) -> ManagerResult<String> {
    info!("extended_config: {}", config_type);
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
