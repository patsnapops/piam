use std::env;

use arc_swap::ArcSwap;
use once_cell::sync::Lazy;

pub type GlobalString = Lazy<ArcSwap<String>>;
pub type GlobalStaticStr = Lazy<ArcSwap<&'static str>>;

pub static CLUSTER_ENV: GlobalString =
    Lazy::new(|| string_var_with_default("CLUSTER_ENV", "Unset"));

pub static META_KEY: GlobalString =
    GlobalString::new(|| string_var_with_default("META_KEY", "0x5F3759DF"));

pub fn dev_mode() -> bool {
    env::args().nth(1) == Some("dev".into())
}

pub fn string_var_with_default(name: &str, default: &str) -> ArcSwap<String> {
    let val = match env::var(name) {
        Ok(s) => s,
        Err(_) => default.to_string(),
    };
    ArcSwap::from_pointee(val)
}
