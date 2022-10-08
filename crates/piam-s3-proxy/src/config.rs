use arc_swap::ArcSwap;
use log::info;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;

use piam_core::config::{CoreConfig, get_resource_string};

pub fn proxy_port() -> u16 {
    if let Some(arg1) = env::args().nth(1) {
        if arg1 == "dev" {
            return 3000;
        }
    }
    8080
}

pub const DEV_PROXY_HOST: &str = "piam-s3-proxy.dev:3000";

pub static S3_CONFIG: Lazy<ArcSwap<S3Config>> =
    Lazy::new(|| ArcSwap::from_pointee(S3Config::default()));

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct S3Config {
    pub proxy_host: String,
    pub actual_host: String,
}

impl S3Config {
    pub async fn get_new() -> Self {
        if let Some(arg1) = env::args().nth(1) {
            if arg1 == "dev" {
                return S3Config {
                    proxy_host: DEV_PROXY_HOST.into(),
                    actual_host: "s3.cn-northwest-1.amazonaws.com.cn".into(),
                };
            }
        }
        let string = get_resource_string("config/s3").await;
        serde_yaml::from_str(&string).unwrap()
    }

    pub async fn update() {
        S3_CONFIG.store(Arc::new(S3Config::get_new().await));
    }

    pub async fn update_all() {
        CoreConfig::update("s3").await;
        Self::update().await;
    }
}
