//! IamContainer is a container of IAM domain entities.
//! It is used to store and query IAM domain entities in memory.

use std::{collections::HashMap, fmt::Debug};

use async_trait::async_trait;
use log::{debug, error, info};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    account::{aws::AwsAccount, AccountId},
    config::{dev_mode, POLICY_MODEL},
    error::{ProxyError, ProxyResult},
    group::{Group, GroupId},
    manager_api::ManagerClient,
    policy::{Policy, PolicyId, Statement},
    principal::{Role, RoleId, User, UserId},
    relation_model::PolicyRelationship,
    state::GetNewState,
};

/// IamContainer store entities.
#[derive(Debug, Default)]
pub struct IamContainer<S: Statement + Debug> {
    /// All accounts by their code, each account one is unique
    accounts: HashMap<String, AwsAccount>,
    /// All users, each one is unique
    users: HashMap<UserId, User>,
    /// All groups, each one is unique
    groups: HashMap<GroupId, Group>,
    /// All policies, each one is unique
    policies: HashMap<PolicyId, Policy<S>>,
    /// In-memory index built from all `AccessKeyUserRelationship`s
    base_access_key_to_user_id: HashMap<String, UserId>,
    /// In-memory index built from all `UserGroupRelationship`s
    user_id_to_group_ids: HashMap<UserId, Vec<GroupId>>,
    /// Relationships of policies and other items
    policy_relationships: Vec<PolicyRelationship>,
    // TODO: Use index from all `PolicyRelatedItems`s to speed up policy querying
    // join IamEntityIds and stuff as hash key of policy relation id
}

/// Struct to use when querying policies from the container.
#[derive(Debug)]
pub struct PolicyQueryParams<'a> {
    pub roles: Option<Vec<&'a Role>>,
    pub user: Option<&'a User>,
    pub groups: Option<Vec<&'a Group>>,
    pub account: &'a AwsAccount,
    pub region: &'a str,
}

#[async_trait]
impl<S: Statement + DeserializeOwned + Debug + Send> GetNewState for IamContainer<S> {
    // TODO: change this to new_from_xxx for the sake of state independent unit test
    async fn new_from_manager(manager: &ManagerClient) -> ProxyResult<Self> {
        let account_vec = manager.get_accounts().await?;
        let users_vec = manager.get_users().await?;
        let groups_vec = manager.get_groups().await?;
        let policy_vec: Vec<Policy<S>> = manager.get_policies(&POLICY_MODEL.load()).await?;

        let user_group_relationships = manager.get_user_group_relationships().await?;
        let policy_relationships = manager.get_policy_relationships().await?;

        let accounts = account_vec
            .into_iter()
            .map(|account| (account.code.clone(), account))
            .collect();
        let users = users_vec
            .clone()
            .into_iter()
            .map(|user| (user.id.clone(), user))
            .collect();
        let groups = groups_vec
            .into_iter()
            .map(|group| (group.id.clone(), group))
            .collect();
        let policies = policy_vec
            .into_iter()
            .map(|policy| (policy.id.clone(), policy))
            .collect();

        let base_access_key_to_user_id = users_vec
            .into_iter()
            .map(|user| (user.base_access_key, user.id))
            .collect();

        let mut user_id_to_group_ids: HashMap<UserId, Vec<GroupId>> = HashMap::default();
        for rel in user_group_relationships {
            let user_id = rel.user_id;
            let group_id = rel.group_id;
            match user_id_to_group_ids.get_mut(&user_id) {
                None => {
                    user_id_to_group_ids.insert(user_id, vec![group_id]);
                }
                Some(group_ids) => {
                    group_ids.push(group_id);
                }
            }
        }

        Ok(IamContainer {
            accounts,
            users,
            groups,
            policies,
            base_access_key_to_user_id,
            user_id_to_group_ids,
            policy_relationships,
        })
    }
}

impl<S: Statement + DeserializeOwned + Debug> IamContainer<S> {
    pub fn find_account_by_code(&self, code: &str) -> ProxyResult<&AwsAccount> {
        self.accounts.get(code).ok_or_else(|| {
            ProxyError::InvalidAccessKey(format!(
                "Account not found for access key with code: {}",
                code
            ))
        })
    }

    pub fn find_user_by_base_access_key(&self, base_access_key: &str) -> ProxyResult<&User> {
        let user_id = self
            .base_access_key_to_user_id
            .get(base_access_key)
            .ok_or_else(|| {
                ProxyError::InvalidAccessKey(format!(
                    "User not found for base access key id: '{}'",
                    base_access_key
                ))
            })?;
        self.users
            .get(user_id)
            .ok_or_else(|| ProxyError::UserNotFound(format!("User not found by id: {}", user_id)))
    }

    pub fn find_groups_by_user(&self, user: &User) -> ProxyResult<Vec<&Group>> {
        let group_ids = self.user_id_to_group_ids.get(&user.id).ok_or_else(|| {
            ProxyError::GroupNotFound(format!("Groups not found for user id: {}", user.id))
        })?;

        group_ids
            .iter()
            .map(|group_id| {
                self.groups.get(group_id).ok_or_else(|| {
                    ProxyError::GroupNotFound(format!("Group not found by id: {}", group_id))
                })
            })
            .collect()
    }

    pub fn find_policies(&self, filter: &PolicyQueryParams) -> ProxyResult<Vec<&Policy<S>>> {
        let policy_ids: Vec<&PolicyId> = self
            .policy_relationships
            .iter()
            .filter(|r| r.account_id == filter.account.id)
            .filter(|r| r.region == filter.region)
            .filter(|r| {
                if let (Some(user_id), Some(user_filter)) = (&r.user_id, filter.user) {
                    *user_id == user_filter.id
                } else {
                    true
                }
            })
            .filter(|r| {
                if let (Some(group_id), Some(group_filter)) = (&r.group_id, &filter.groups) {
                    group_filter.iter().any(|gf| gf.id == *group_id)
                } else {
                    true
                }
            })
            .filter(|r| {
                if let (Some(role_id), Some(role_filter)) = (&r.role_id, &filter.roles) {
                    role_filter.iter().any(|rf| rf.id == *role_id)
                } else {
                    true
                }
            })
            .map(|r| &r.policy_id)
            .collect();

        if policy_ids.is_empty() {
            return Err(ProxyError::MissingPolicy(format!(
                "access denied by missing policy account: {} region: {} groups: {:#?}",
                filter.account.id, filter.region, filter.groups
            )));
        }

        policy_ids
            .into_iter()
            .map(|policy_id| {
                self.policies.get(policy_id).ok_or_else(|| {
                    ProxyError::OtherInternal(format!("Policy not found by id: {}", policy_id))
                })
            })
            .collect()
    }
}
