use std::{env, sync::Arc};

use arc_swap::ArcSwap;
use log::info;
use once_cell::sync::Lazy;
use piam_tracing::logger::LogHandle;
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

pub static PROXY_TYPE: Lazy<ArcSwap<&'static str>> = Lazy::new(|| ArcSwap::from_pointee("[Unset]"));
pub static POLICY_MODEL: Lazy<ArcSwap<&'static str>> = Lazy::new(|| ArcSwap::from_pointee("Unset"));
pub const STATE_UPDATE_INTERVAL: u64 = 60;

pub fn set_proxy_type(proxy_type: &'static str) {
    PROXY_TYPE.store(Arc::new(proxy_type));
}

pub fn set_policy_model(policy_model: &'static str) {
    POLICY_MODEL.store(Arc::new(policy_model));
}

pub static PIAM_MANAGER_ADDRESS: Lazy<ArcSwap<String>> =
    Lazy::new(|| string_var_with_default("PIAM_MANAGER_ADDRESS", "http://localhost:8080"));

fn string_var_with_default(name: &str, default: &str) -> ArcSwap<String> {
    let val = match env::var(name) {
        Ok(s) => s,
        Err(_) => default.to_string(),
    };
    info!("{}: {}", name, val);
    ArcSwap::from_pointee(val)
}
