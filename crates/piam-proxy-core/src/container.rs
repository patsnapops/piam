//! IamContainer is a container of IAM domain entities.
//! It is used to store and query IAM domain entities in memory.

use std::{collections::HashMap, fmt::Debug};

use async_trait::async_trait;
use piam_common::{manager_api::CONDITION_MODEL, ANY};
use serde::de::DeserializeOwned;

use crate::{
    account::{aws::AwsAccount, AccountId},
    config::POLICY_MODEL,
    error::{esome, ProxyError, ProxyResult},
    group::{Group, GroupId},
    manager_api::ManagerClient,
    policy::{condition::ConditionPolicy, Modeled, Policy, PolicyId},
    principal::{Role, RoleId, User, UserId},
    relation_model::PolicyRelationship,
    state::GetNewState,
};

/// IamContainer store entities.
#[derive(Debug, Default)]
pub struct IamContainer<P: Modeled> {
    /// All accounts by their code, each account one is unique
    accounts: HashMap<String, AwsAccount>,
    /// All users, each one is unique
    users: HashMap<UserId, User>,
    /// All groups, each one is unique
    groups: HashMap<GroupId, Group>,
    /// Policies for user input, each one is unique
    user_input_policies: HashMap<PolicyId, Policy<P>>,
    /// Policies for condition, each one is unique
    condition_policies: HashMap<PolicyId, Policy<ConditionPolicy>>,
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
pub struct PolicyFilterParams<'a> {
    pub roles: Option<Vec<&'a Role>>,
    pub user: Option<&'a User>,
    pub groups: Option<Vec<&'a Group>>,
    pub account: &'a AwsAccount,
    pub target_region: &'a str,
}

impl<'a> PolicyFilterParams<'a> {
    pub fn new_with_default(
        groups: Vec<&'a Group>,
        account: &'a AwsAccount,
        target_region: &'a str,
    ) -> Self {
        PolicyFilterParams {
            roles: None,
            user: None,
            groups: Some(groups),
            account,
            target_region,
        }
    }
}

#[derive(Debug)]
pub struct FoundPolicies<'a, P: Modeled> {
    pub condition: Vec<&'a Policy<ConditionPolicy>>,
    pub user_input: Vec<&'a Policy<P>>,
}

#[async_trait]
impl<P: Modeled + DeserializeOwned + Send> GetNewState for IamContainer<P> {
    // TODO: change this to new_from_xxx for the sake of state independent unit test
    async fn new_from_manager(manager: &ManagerClient) -> ProxyResult<Self> {
        let account_vec = manager.get_accounts().await?;
        let users_vec = manager.get_users().await?;
        let groups_vec = manager.get_groups().await?;
        let user_input_policy_vec: Vec<Policy<P>> =
            manager.get_policies_by_model(&POLICY_MODEL.load()).await?;
        let condition_policy_vec = manager.get_policies_by_model(CONDITION_MODEL).await?;

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
        let user_input_policies = user_input_policy_vec
            .into_iter()
            .map(|policy| (policy.id.clone(), policy))
            .collect();
        let condition_policies = condition_policy_vec
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
            user_input_policies,
            condition_policies,
            base_access_key_to_user_id,
            user_id_to_group_ids,
            policy_relationships,
        })
    }
}

impl<P: Modeled> IamContainer<P> {
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

    pub fn find_policies(&self, filter: &PolicyFilterParams) -> ProxyResult<FoundPolicies<P>> {
        let relations: Vec<&PolicyRelationship> = self
            .policy_relationships
            .iter()
            .filter(|r| Self::account_filter(&r.account_id, filter.account))
            .filter(|r| Self::region_filter(&r.region, &filter.target_region))
            .filter(|r| Self::user_filter(r.user_id.as_ref(), filter.user))
            .filter(|r| Self::group_filter(r.group_id.as_ref(), filter.groups.as_ref()))
            .filter(|r| Self::role_filter(r.role_id.as_ref(), filter.roles.as_ref()))
            .collect();

        if relations.is_empty() {
            return Err(ProxyError::MissingPolicy(format!(
                "access denied by missing policy, account: {} region: {} groups: {:#?}",
                filter.account.id, filter.target_region, filter.groups
            )));
        }

        let mut condition: Vec<&Policy<ConditionPolicy>> = Vec::new();
        let mut user_input: Vec<&Policy<P>> = Vec::new();
        for relation in relations {
            match relation.policy_model.as_str() {
                CONDITION_MODEL => {
                    let p = esome(self.condition_policies.get(&relation.policy_id));
                    condition.push(p);
                }
                user_input_model if user_input_model == POLICY_MODEL.load().to_string() => {
                    let p = esome(self.user_input_policies.get(&relation.policy_id));
                    user_input.push(p);
                }
                other => {
                    return Err(ProxyError::OtherInternal(format!(
                        "unknown policy model found: {}",
                        other
                    )));
                }
            };
        }

        Ok(FoundPolicies {
            condition,
            user_input,
        })
    }

    fn account_filter(record_id: &AccountId, param: &AwsAccount) -> bool {
        record_id == &param.id || record_id == ANY
    }

    fn region_filter(record: &str, param: &str) -> bool {
        record == param || record == ANY
    }

    fn user_filter(record_id: Option<&UserId>, param: Option<&User>) -> bool {
        match param {
            None => true,
            Some(p) => match record_id {
                None => true,
                Some(rid) => (*rid == p.id) || rid == ANY,
            },
        }
    }

    //noinspection DuplicatedCode
    fn group_filter(record_id: Option<&GroupId>, param: Option<&Vec<&Group>>) -> bool {
        match param {
            None => true,
            Some(p) => match record_id {
                None => true,
                Some(rid) => p.iter().any(|pg| *rid == pg.id) || rid == ANY,
            },
        }
    }

    //noinspection DuplicatedCode
    fn role_filter(record_id: Option<&RoleId>, param: Option<&Vec<&Role>>) -> bool {
        match param {
            None => true,
            Some(p) => match record_id {
                None => true,
                Some(rid) => p.iter().any(|pr| *rid == pr.id) || rid == ANY,
            },
        }
    }
}
