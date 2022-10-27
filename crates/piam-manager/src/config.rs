use std::env;

use arc_swap::ArcSwap;
use log::info;
use once_cell::sync::Lazy;

pub static REDIS_ADDRESS: Lazy<ArcSwap<String>> = Lazy::new(|| {
    let addr = match env::var("REDIS_ADDRESS") {
        Ok(s) => s,
        Err(_) => "redis://localhost/1".to_string(),
    };
    info!("REDIS_ADDRESS: {}", addr);
    ArcSwap::from_pointee(addr)
});

pub fn dev_mode() -> bool {
    env::args().nth(1) == Some("dev".into())
}

pub fn port() -> u16 {
    if dev_mode() {
        return 8080;
    }
    80
}
