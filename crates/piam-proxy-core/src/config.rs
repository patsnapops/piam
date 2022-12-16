use std::{env, sync::Arc};

use arc_swap::ArcSwap;
use log::info;
use once_cell::sync::Lazy;

pub const CN_NORTHWEST_1: &str = "cn-northwest-1";
pub const US_EAST_1: &str = "us-east-1";
pub const AP_SHANGHAI: &str = "ap-shanghai";
pub const NA_ASHBURN: &str = "na-ashburn";

pub static PROXY_TYPE: Lazy<ArcSwap<&'static str>> = Lazy::new(|| ArcSwap::from_pointee("[Unset]"));
pub static POLICY_MODEL: Lazy<ArcSwap<&'static str>> = Lazy::new(|| ArcSwap::from_pointee("Unset"));
pub static PIAM_MANAGER_ADDRESS: Lazy<ArcSwap<String>> =
    Lazy::new(|| string_var_with_default("PIAM_MANAGER_ADDRESS", "http://localhost:8080"));
pub const STATE_UPDATE_INTERVAL: u64 = 10;

pub fn set_constants(proxy_type: &'static str, policy_model: &'static str) {
    PROXY_TYPE.store(Arc::new(proxy_type));
    POLICY_MODEL.store(Arc::new(policy_model));
    info!("PROXY_TYPE: {}", proxy_type);
    info!("POLICY_MODEL: {}", policy_model);
    info!("CLUSTER_ENV: {}", CLUSTER_ENV.load());
    info!("PIAM_MANAGER_ADDRESS: {}", PIAM_MANAGER_ADDRESS.load());
}

pub trait ParserConfig {}

// TODO: below should be moved to a more general crate

pub static CLUSTER_ENV: Lazy<ArcSwap<String>> =
    Lazy::new(|| string_var_with_default("CLUSTER_ENV", "Unset"));

pub fn dev_mode() -> bool {
    env::args().nth(1) == Some("dev".into())
}

pub fn server_port() -> u16 {
    if dev_mode() {
        return 80;
    }
    80
}

fn string_var_with_default(name: &str, default: &str) -> ArcSwap<String> {
    let val = match env::var(name) {
        Ok(s) => s,
        Err(_) => default.to_string(),
    };
    ArcSwap::from_pointee(val)
}
