use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use log::{error, info};

use crate::store::get_resource_string;

type StringResult = Result<String, AppError>;

pub async fn health() -> impl IntoResponse {
    "OK"
}

pub async fn get_principals() -> StringResult {
    info!("get_principals");
    Ok(get_resource_string("principals").await)
}

pub async fn get_policies(Path(kind): Path<String>) -> StringResult {
    info!("get_policies: {}", kind);
    Ok(get_resource_string(format!("policies:{}", kind).as_str()).await)
}

pub async fn get_amz_sign_params(Path((service, region)): Path<(String, String)>) -> StringResult {
    info!("get_amz_sign_params:{}:{}", service, region);
    Ok(get_resource_string(format!("amz_sign_params:{}:{}", service, region).as_str()).await)
}

pub async fn get_config(Path((service, region)): Path<(String, String)>) -> StringResult {
    info!("get_config:{}:{}", service, region);
    Ok(get_resource_string(format!("config:{}:{}", service, region).as_str()).await)
}

pub enum AppError {
    GetRegionErr(String),
    Panic(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = match self {
            AppError::GetRegionErr(e) => e,
            AppError::Panic(e) => e,
        };
        error!("AppError: {}", body);
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

#[cfg(test)]
mod tests {}
