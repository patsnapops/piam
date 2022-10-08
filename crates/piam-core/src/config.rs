use std::env;
use std::sync::Arc;

use arc_swap::ArcSwap;
use hyper::Body;
use hyper::client::HttpConnector;
use log::info;
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use piam_tracing::logger::LogHandle;

pub static PIAM_MANAGER_ADDRESS: Lazy<ArcSwap<String>> = Lazy::new(|| {
    let addr = match env::var("PIAM_MANAGER_ADDRESS") {
        Ok(s) => s,
        Err(_) => "http://localhost:8080".to_string(),
    };
    info!("PIAM_MANAGER_ADDRESS: {}", addr);
    ArcSwap::from_pointee(addr)
});

pub static CORE_CONFIG: Lazy<ArcSwap<CoreConfig>> =
    Lazy::new(|| ArcSwap::from_pointee(CoreConfig::default()));

#[derive(Debug, Default)]
pub struct CoreConfig {
    pub log_handle: Option<LogHandle>,
    pub client: hyper::Client<HttpConnector, Body>,
    pub amz_sign_params: AmzSignParams,
}

impl CoreConfig {
    pub async fn get_new(service: &str) -> Self {
        let string = get_resource_string(&format!("amz_sign_params/{}", service)).await;
        let amz_sign_params: AmzSignParams = serde_yaml::from_str(&string).unwrap();
        CoreConfig {
            log_handle: None,
            client: Default::default(),
            amz_sign_params,
        }
    }

    pub async fn update(service: &str) {
        CORE_CONFIG.store(Arc::new(CoreConfig::get_new(service).await));
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AmzSignParams {
    pub access_key: String,
    pub secret_key: String,
    pub region: String,
    pub service: String,
}

pub async fn get_resource_string(key: &str) -> String {
    Client::new()
        .get(format!("{}/{}", *PIAM_MANAGER_ADDRESS, key).as_str())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn ser_one_amz_sign_params() {
        let amz_sign_params = serde_yaml::to_string(&AmzSignParams {
            access_key: "ak".to_string(),
            secret_key: "sk".to_string(),
            region: "cn".to_string(),
            service: "s3".to_string(),
        })
        .unwrap();
        println!("{}", amz_sign_params);
    }
}
