use busylib::config::{dev_mode, env_var_with_default, GlobalString};

pub static REDIS_ADDRESS: GlobalString =
    GlobalString::new(|| env_var_with_default("REDIS_ADDRESS", "redis://localhost/1"));

pub fn port() -> u16 {
    if dev_mode() {
        return 8080;
    }
    80
}
