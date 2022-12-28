use std::sync::Arc;

use arc_swap::ArcSwap;
use busylib::config::{dev_mode, env_var_with_default, GlobalStaticStr, GlobalString};
use log::{error, info};
use once_cell::sync::Lazy;
use piam_core::{
    account::aws::AwsAccount,
    group::Group,
    policy::{condition::ConditionPolicy, Modeled, Policy},
    principal::User,
    relation_model::{PolicyRelationship, UserGroupRelationship},
};

pub static PROXY_TYPE: GlobalStaticStr = Lazy::new(|| ArcSwap::from_pointee(UNSET));
pub static POLICY_MODEL: GlobalStaticStr = Lazy::new(|| ArcSwap::from_pointee(UNSET));
pub static EXTENDED_CONFIG_TYPE: GlobalStaticStr = Lazy::new(|| ArcSwap::from_pointee(UNSET));
pub static PROXY_REGION: GlobalString = GlobalString::new(|| env_var_with_default("REGION", UNSET));
pub static PROXY_ENV: GlobalString = GlobalString::new(|| env_var_with_default("ENV", UNSET));
pub static PIAM_MANAGER_ADDRESS: GlobalString =
    GlobalString::new(|| env_var_with_default("PIAM_MANAGER_ADDRESS", UNSET));

pub const UNSET: &str = "Unset";
pub const STATE_UPDATE_INTERVAL: u64 = 10;

#[derive(Debug, Default)]
pub struct CoreConfig<P: Modeled> {
    pub accounts: Vec<AwsAccount>,
    pub users: Vec<User>,
    pub groups: Vec<Group>,
    pub user_input_policies: Vec<Policy<P>>,
    pub condition_policies: Vec<Policy<ConditionPolicy>>,
    pub user_group_relationships: Vec<UserGroupRelationship>,
    pub policy_relationships: Vec<PolicyRelationship>,
}

pub fn set_constants(
    proxy_type: &'static str,
    policy_model: &'static str,
    extended_config_type: &'static str,
) {
    PROXY_TYPE.store(Arc::new(proxy_type));
    POLICY_MODEL.store(Arc::new(policy_model));
    EXTENDED_CONFIG_TYPE.store(Arc::new(extended_config_type));

    if dev_mode() {
        PIAM_MANAGER_ADDRESS.store(Arc::new("http://localhost:8080".to_string()));
    } else {
        let required = vec![&PROXY_REGION, &PROXY_ENV, &PIAM_MANAGER_ADDRESS];
        required
            .iter()
            .any(|s| {
                let v = s.load();
                v.to_string().is_empty()
                    || v.to_string() == UNSET
                    || !v.chars().all(|c| c.is_ascii_graphic())
            })
            .then(|| {
                error!("required environment args not valid: {:?}", required);
                std::process::exit(1);
            });
    }
    info!("PROXY_TYPE: {}", proxy_type);
    info!("POLICY_MODEL: {}", policy_model);
    info!("EXTENDED_CONFIG_TYPE: {}", extended_config_type);
    info!("PROXY_REGION: {}", PROXY_REGION.load());
    info!("PROXY_ENV: {}", PROXY_ENV.load());
    info!("PIAM_MANAGER_ADDRESS: {}", PIAM_MANAGER_ADDRESS.load());
}

#[inline]
pub fn proxy_region_env() -> String {
    format!("{}-{}", PROXY_REGION.load(), PROXY_ENV.load())
}

pub fn server_port() -> u16 {
    if dev_mode() {
        return 80;
    }
    80
}
