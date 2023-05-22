use busylib::config::{dev_mode, env_var_with_default, GlobalString};
use std::collections::HashMap;
use lazy_static::lazy_static;
use piam_core::manager_api_constant::*;

pub static REDIS_ADDRESS: GlobalString =
    GlobalString::new(|| env_var_with_default("REDIS_ADDRESS", "redis://localhost/1"));

pub fn port() -> u16 {
    if dev_mode() {
        return 8080;
    }
    80
}

pub static PORTAL_ADDRESS: GlobalString =
    GlobalString::new(|| env_var_with_default("PORTAL_ADDRESS", "http://localhost:80"));

pub static PORTAL_API_VERSION: GlobalString =
    GlobalString::new(|| env_var_with_default("PORTAL_API_VERSION", "v2023-03"));

lazy_static! {
    pub static ref PORTAL_API_PATH: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert(ACCOUNTS, "accounts");
        map.insert(USERS, "users");
        map.insert(GROUPS, "groups");
        map.insert(POLICIES, "policies");
        map.insert(CONDITION, "conditions");
        map.insert(USER_GROUP_RELATIONSHIPS, "relationships/user_group");
        map.insert(POLICY_RELATIONSHIPS, "relationships/policy");
        map.insert(EXTENDED_CONFIG, "extended_configurations");
        map
    };
}
