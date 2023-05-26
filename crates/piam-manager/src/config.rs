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

pub static PIAM_HTTP_RESOURCE_ADDRESS: GlobalString =
    GlobalString::new(|| env_var_with_default("PIAM_HTTP_RESOURCE_ADDRESS", "http://localhost:8080"));

pub static PIAM_HTTP_RESOURCE_API_VERSION: GlobalString =
    GlobalString::new(|| env_var_with_default("PIAM_HTTP_RESOURCE_API_VERSION", "v2023-03"));
pub static PIAM_HTTP_RESOURCE_URL_PREFIX: GlobalString =
    GlobalString::new(|| env_var_with_default("PIAM_HTTP_RESOURCE_URL_PREFIX",
                                              format!("{}/{}/{}",
                                                      PIAM_HTTP_RESOURCE_ADDRESS.load(),
                                                      PIAM_HTTP_RESOURCE_API_VERSION.load(),
                                                      "piam").as_str()));

lazy_static! {
    pub static ref PIAM_HTTP_PATH_REDIS_KEY_MAPPER: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert(USERS, "users");
        map.insert(GROUPS, "groups");
        map.insert(ACCOUNTS, "accounts");
        map.insert(POLICY_RELATIONSHIPS, "relationships/policy");
        map.insert(USER_GROUP_RELATIONSHIPS, "relationships/user_group");
        map.insert(
            Box::leak(format!("{}:{}", POLICIES, CONDITION).into_boxed_str()),
            "policies/kind/Condition"
        );
        map.insert(
            Box::leak(format!("{}:{}", EXTENDED_CONFIG, S3).into_boxed_str()),
            "extended_configurations"
        );
        map.insert(
            Box::leak(format!("{}:{}", POLICIES, OBJECT_STORAGE).into_boxed_str()),
            "policies/kind/ObjectStorage"
        );
        map
    };
}
