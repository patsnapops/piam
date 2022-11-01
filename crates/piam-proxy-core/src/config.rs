use std::{env, sync::Arc};

use arc_swap::ArcSwap;
use hyper::{client::HttpConnector, Body};
use log::info;
use once_cell::sync::Lazy;
use piam_tracing::logger::LogHandle;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub fn dev_mode() -> bool {
    env::args().nth(1) == Some("dev".into())
}

pub fn proxy_port() -> u16 {
    if dev_mode() {
        return 80;
    }
    80
}

pub static PIAM_MANAGER_ADDRESS: Lazy<ArcSwap<String>> =
    Lazy::new(|| string_var_with_default("PIAM_MANAGER_ADDRESS", "http://localhost:8080"));

pub static REGION: Lazy<ArcSwap<String>> = Lazy::new(|| {
    // AWS_REGION is provided by patsnap CI/CD infra, can be overwritten by REGION
    let mut region = "cn-northwest-1".to_string();
    if let Ok(r) = env::var("AWS_REGION") {
        region = r
    }
    if let Ok(r) = env::var("REGION") {
        region = r
    }
    info!("REGION: {}", region);
    ArcSwap::from_pointee(region)
});

fn string_var_with_default(name: &str, default: &str) -> ArcSwap<String> {
    let val = match env::var(name) {
        Ok(s) => s,
        Err(_) => default.to_string(),
    };
    info!("{}: {}", name, val);
    ArcSwap::from_pointee(val)
}

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
        let key = format!("amz_sign_params/{}/{}", service, REGION.load());
        let string = get_resource_string(&key).await;
        let amz_sign_params: AmzSignParams = serde_yaml::from_str(&string)
            .expect("AmzSignParams deser error in CoreConfig::get_new");
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
    let url = format!("{}/{}", PIAM_MANAGER_ADDRESS.load(), key);
    // A native-tls/rust-tls related issue: Should set default-features = false, features = ["rustls-tls"]
    // for reqwest in Cargo.toml, otherwise Segmentation fault (core dumped) will happen when Client::new().
    let client = Client::new();
    match client.get(&url).send().await {
        Ok(r) => r.text().await.expect("get_resource_string text error"),
        Err(_) => {
            dbg!("retry one more time to get resource string");
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            client
                .get(&url)
                .send()
                .await
                .expect("get_resource_string send error")
                .text()
                .await
                .expect("get_resource_string text error")
        }
    }
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
