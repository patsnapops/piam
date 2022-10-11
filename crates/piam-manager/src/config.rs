use std::env;

use arc_swap::ArcSwap;
use log::info;
use once_cell::sync::Lazy;

pub static REDIS_ADDRESS: Lazy<ArcSwap<String>> = Lazy::new(|| {
    let addr = match env::var("REDIS_ADDRESS") {
        Ok(s) => s,
        Err(_) => "redis://dev-redis.patsnap.info:30070/1".to_string(),
    };
    info!("REDIS_ADDRESS: {}", addr);
    ArcSwap::from_pointee(addr)
});
