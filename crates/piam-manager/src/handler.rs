use axum::extract::Path;
use axum_client_ip::ClientIp;
use axum_core::response::IntoResponse;
use log::info;
use std::net::IpAddr;

use crate::store::get_resource_string;

pub async fn health() -> impl IntoResponse {
    info!("health");
    "OK"
}

pub async fn get_principals() -> impl IntoResponse {
    info!("get_principals");
    get_resource_string("principals").await
}

pub async fn get_policies(Path(kind): Path<String>) -> impl IntoResponse {
    info!("get_policies: {}", kind);
    get_resource_string(format!("policies:{}", kind).as_str()).await
}

pub async fn get_amz_sign_params(
    Path(service): Path<String>,
    ClientIp(ip): ClientIp,
) -> impl IntoResponse {
    info!("get_amz_sign_params: {} from: {}", service, ip);
    let region = get_region(ClientIp(ip));
    get_resource_string(format!("amz_sign_params:{}:{}", service, region).as_str()).await
}

pub async fn get_config(Path(proxy): Path<String>, ClientIp(ip): ClientIp) -> impl IntoResponse {
    info!("get_config: {} from: {}", proxy, ip);
    let region = get_region(ClientIp(ip));
    get_resource_string(format!("config:{}:{}", proxy, region).as_str()).await
}

fn get_region<'a>(ClientIp(ip): ClientIp) -> &'a str {
    match ip {
        IpAddr::V4(v4) => {
            if v4.is_loopback() {
                // for dev
                "cn-northwest-1"
            } else {
                panic!("{} not yet supported", ip)
            }
        }
        IpAddr::V6(_) => panic!("IPV6 not yet supported"),
    }
}
