use busylib::config::{string_var_with_default, GlobalString};

pub static META_KEY: GlobalString =
    GlobalString::new(|| string_var_with_default("META_KEY", "0x5F3759DF"));
