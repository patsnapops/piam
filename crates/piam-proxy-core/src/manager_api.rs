use std::fmt::Debug;

use serde::de::DeserializeOwned;

use crate::{
    account::aws::AwsAccount,
    config::{dev_mode, PIAM_MANAGER_ADDRESS},
    error::{deserialize, ProxyResult},
    group::Group,
    policy::{Policy, Statement},
    principal::User,
    relation_model::{PolicyRelationship, UserGroupRelationship},
};

#[derive(Debug, Default)]
pub struct ManagerClient {
    http_client: reqwest::Client,
}

impl ManagerClient {
    pub async fn get_accounts(&self) -> ProxyResult<Vec<AwsAccount>> {
        let mut account_vec: Vec<AwsAccount> = self.get_resource("accounts").await?;
        if dev_mode() {
            account_vec = account_vec
                .into_iter()
                .map(|mut account| {
                    account.id = account.id.replace("dev", "prod");
                    account
                })
                .collect()
        }
        Ok(account_vec)
    }

    pub async fn get_users(&self) -> ProxyResult<Vec<User>> {
        self.get_resource("users").await
    }

    pub async fn get_groups(&self) -> ProxyResult<Vec<Group>> {
        self.get_resource("groups").await
    }

    pub async fn get_policies<S: Statement + DeserializeOwned + Debug>(
        &self,
        policy_model: &str,
    ) -> ProxyResult<Vec<Policy<S>>> {
        self.get_resource(&format!("policies/{}", policy_model))
            .await
    }

    pub async fn get_user_group_relationships(&self) -> ProxyResult<Vec<UserGroupRelationship>> {
        self.get_resource("user_group_relationships").await
    }

    pub async fn get_policy_relationships(&self) -> ProxyResult<Vec<PolicyRelationship>> {
        self.get_resource("policy_relationships").await
    }

    pub async fn get_extended_config<T: DeserializeOwned>(
        &self,
        extended_config_key: &str,
    ) -> ProxyResult<T> {
        self.get_resource(&format!("extended_config/{}", extended_config_key))
            .await
    }

    async fn get_resource<T: DeserializeOwned>(&self, key: &str) -> ProxyResult<T> {
        let resource_string = self.get_resource_string(key).await?;
        let resource =
            serde_yaml::from_str(&resource_string).map_err(|e| deserialize(resource_string, e))?;
        Ok(resource)
    }

    async fn get_resource_string(&self, key: &str) -> ProxyResult<String> {
        let url = format!("{}/{}", PIAM_MANAGER_ADDRESS.load(), key);
        // A native-tls/rust-tls related issue:
        // default-features = false, features = ["rustls-tls"] for reqwest should be set in Cargo.toml,
        // otherwise Segmentation fault (core dumped) may occur when creating a new reqwest client.
        let client = &self.http_client;
        let resource = client.get(&url).send().await?.text().await?;
        Ok(resource)
    }
}
