use std::sync::Arc;

use arc_swap::ArcSwap;
use busylib::config::{dev_mode, string_var_with_default, GlobalStaticStr, GlobalString};
use log::info;
use once_cell::sync::Lazy;

pub static PROXY_TYPE: GlobalStaticStr = Lazy::new(|| ArcSwap::from_pointee("[Unset]"));
pub static POLICY_MODEL: GlobalStaticStr = Lazy::new(|| ArcSwap::from_pointee("[Unset]"));
pub static CLUSTER_ENV: GlobalString =
    GlobalString::new(|| string_var_with_default("CLUSTER_ENV", "Unset"));
pub static PIAM_MANAGER_ADDRESS: GlobalString =
    GlobalString::new(|| string_var_with_default("PIAM_MANAGER_ADDRESS", "http://localhost:8080"));
pub static META_KEY: GlobalString =
    GlobalString::new(|| string_var_with_default("META_KEY", "0x5F3759DF"));
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

pub fn server_port() -> u16 {
    if dev_mode() {
        return 80;
    }
    80
}
