use std::{env, sync::Arc};

use arc_swap::ArcSwap;
use once_cell::sync::Lazy;
use piam_proxy_core::config::{dev_mode, get_resource_string, CoreConfig, REGION};
use serde::{Deserialize, Serialize};

pub const DEV_PROXY_HOST: &str = "piam-s3-proxy.dev";
pub const SERVICE: &str = "s3";

pub static S3_CONFIG: Lazy<ArcSwap<S3Config>> =
    Lazy::new(|| ArcSwap::from_pointee(S3Config::default()));

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct S3Config {
    pub proxy_hosts: Vec<String>,
    pub actual_host: String,
}

impl S3Config {
    pub async fn get_new() -> Self {
        let key = format!("config/{}/{}", SERVICE, REGION.load());
        let string = get_resource_string(&key).await;
        let mut config: S3Config = serde_yaml::from_str(&string).unwrap();
        if dev_mode() {
            config.proxy_hosts.push(DEV_PROXY_HOST.to_string());
        }
        config
    }

    pub async fn update() {
        S3_CONFIG.store(Arc::new(S3Config::get_new().await));
    }

    pub async fn update_all() {
        CoreConfig::update(SERVICE).await;
        Self::update().await;
    }

    pub fn find_proxy_host(&self, host: &str) -> &String {
        self.proxy_hosts
            .iter()
            .find(|&v| host.ends_with(v))
            .expect("host should ends with one of proxy_hosts")
    }
}
