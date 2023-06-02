use busylib::config::{env_var_with_default, GlobalString};

pub static META_KEY: GlobalString =
    GlobalString::new(|| env_var_with_default("META_KEY", "YOUR_META_KEY"));
