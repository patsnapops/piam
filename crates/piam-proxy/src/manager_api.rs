use std::fmt::Debug;

use piam_core::{
    account::aws::AwsAccount,
    crypto::decrypt,
    group::Group,
    manager_api_constant::*,
    policy::{Modeled, Policy},
    principal::User,
    relation_model::{PolicyRelationship, UserGroupRelationship},
};
use serde::de::DeserializeOwned;

use crate::{
    config::PIAM_MANAGER_ADDRESS,
    error::{deserialize, ProxyResult},
};

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
