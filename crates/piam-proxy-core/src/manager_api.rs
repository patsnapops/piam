use std::fmt::Debug;

use serde::de::DeserializeOwned;

use crate::{
    account::aws::AwsAccount,
    config::PIAM_MANAGER_ADDRESS,
    crypto::decrypt,
    error::{deserialize, ProxyResult},
    group::Group,
    manager_api::constants::*,
    policy::{Modeled, Policy},
    principal::User,
    relation_model::{PolicyRelationship, UserGroupRelationship},
};

pub mod constants {
    pub const VERSION: &str = "v2";

    pub const ACCOUNTS: &str = "accounts";

    pub const USERS: &str = "users";

    pub const GROUPS: &str = "groups";

    pub const POLICIES: &str = "policies";
    /// for manager use only
    pub const POLICY_MODEL: &str = "policy_model";
    pub const CONDITION_MODEL: &str = "Condition";

    pub const USER_GROUP_RELATIONSHIPS: &str = "user_group_relationships";

    pub const POLICY_RELATIONSHIPS: &str = "policy_relationships";

    pub const EXTENDED_CONFIG: &str = "extended_config";
    /// for manager use only
    pub const CONFIG_TYPE: &str = "config_type";

    /// for proxy use only
    pub fn policies_path(policy_model: &str) -> String {
        format!("{}/{}", POLICIES, policy_model)
    }

    /// for proxy use only
    pub fn extended_config_path(config_type: &str) -> String {
        format!("{}/{}", EXTENDED_CONFIG, config_type)
    }
}

#[derive(Debug, Default)]
pub struct ManagerClient {
    http_client: reqwest::Client,
}

impl ManagerClient {
    pub async fn get_accounts(&self) -> ProxyResult<Vec<AwsAccount>> {
        self.get_resource(ACCOUNTS).await
    }

    pub async fn get_users(&self) -> ProxyResult<Vec<User>> {
        self.get_resource(USERS).await
    }

    pub async fn get_groups(&self) -> ProxyResult<Vec<Group>> {
        self.get_resource(GROUPS).await
    }

    pub async fn get_policies_by_model<P: Modeled + DeserializeOwned>(
        &self,
        policy_model: &str,
    ) -> ProxyResult<Vec<Policy<P>>> {
        self.get_resource(&policies_path(policy_model)).await
    }

    pub async fn get_user_group_relationships(&self) -> ProxyResult<Vec<UserGroupRelationship>> {
        self.get_resource(USER_GROUP_RELATIONSHIPS).await
    }

    pub async fn get_policy_relationships(&self) -> ProxyResult<Vec<PolicyRelationship>> {
        self.get_resource(POLICY_RELATIONSHIPS).await
    }

    pub async fn get_extended_config<T: DeserializeOwned>(
        &self,
        config_type: &str,
    ) -> ProxyResult<T> {
        self.get_resource(&extended_config_path(config_type)).await
    }

    async fn get_resource<T: DeserializeOwned>(&self, path: &str) -> ProxyResult<T> {
        // manually decrypt for HTTP
        let resource_string = decrypt(self.get_resource_string(path).await?);
        let resource = serde_yaml::from_str(&resource_string)
            .map_err(|e| deserialize(path, resource_string, e))?;
        Ok(resource)
    }

    async fn get_resource_string(&self, path: &str) -> ProxyResult<String> {
        let url = format!("{}/{}/{}", PIAM_MANAGER_ADDRESS.load(), VERSION, path);
        // A native-tls/rust-tls related issue:
        // default-features = false, features = ["rustls-tls"] for reqwest should be set in Cargo.toml,
        // otherwise Segmentation fault (core dumped) may occur when creating a new reqwest client.
        let client = &self.http_client;
        let resource = client.get(&url).send().await?.text().await?;
        Ok(resource)
    }
}
