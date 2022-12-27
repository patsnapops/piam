use std::sync::Arc;

use arc_swap::ArcSwap;
use busylib::config::{dev_mode, string_var_with_default, GlobalStaticStr, GlobalString};
use log::info;
use once_cell::sync::Lazy;
use piam_core::{
    account::aws::AwsAccount,
    group::Group,
    policy::{condition::ConditionPolicy, Modeled, Policy},
    principal::User,
    relation_model::{PolicyRelationship, UserGroupRelationship},
};

pub static PROXY_TYPE: GlobalStaticStr = Lazy::new(|| ArcSwap::from_pointee("[Unset]"));
pub static POLICY_MODEL: GlobalStaticStr = Lazy::new(|| ArcSwap::from_pointee("[Unset]"));
pub static EXTENDED_CONFIG_TYPE: GlobalStaticStr = Lazy::new(|| ArcSwap::from_pointee("[Unset]"));
pub static CLUSTER_ENV: GlobalString =
    GlobalString::new(|| string_var_with_default("CLUSTER_ENV", "Unset"));
pub static PIAM_MANAGER_ADDRESS: GlobalString =
    GlobalString::new(|| string_var_with_default("PIAM_MANAGER_ADDRESS", "http://localhost:8080"));

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
    config_type: &'static str,
) {
    PROXY_TYPE.store(Arc::new(proxy_type));
    POLICY_MODEL.store(Arc::new(policy_model));
    EXTENDED_CONFIG_TYPE.store(Arc::new(config_type));
    info!("PROXY_TYPE: {}", proxy_type);
    info!("POLICY_MODEL: {}", policy_model);
    info!("EXTENDED_CONFIG_TYPE: {}", config_type);
    info!("CLUSTER_ENV: {}", CLUSTER_ENV.load());
    info!("PIAM_MANAGER_ADDRESS: {}", PIAM_MANAGER_ADDRESS.load());
}

pub fn server_port() -> u16 {
    if dev_mode() {
        return 80;
    }
    80
}
