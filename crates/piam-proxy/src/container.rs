//! IamContainer is a container of IAM domain entities.
//! It is used to store and query IAM domain entities in memory.

use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
};

use busylib::{prelude::EnhancedUnwrap, ANY};
use piam_core::{
    account::aws::AwsAccount,
    condition::input::Condition,
    group::{Group, GroupId},
    manager_api_constant::CONDITION,
    policy::{condition::ConditionPolicy, Modeled, Policy, PolicyId},
    principal::{Role, User, UserId},
    relation_model::PolicyRelationship,
    IamIdentity,
};
use serde::de::DeserializeOwned;

use crate::{
    config::{CoreConfig, POLICY_MODEL, PROXY_ENV, PROXY_REGION},
    error::{ProxyError, ProxyResult},
    state::CoreState,
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
    /// Policies for condition, each one is unique
    condition_policies: HashMap<PolicyId, Policy<ConditionPolicy>>,
    /// Policies for user input, each one is unique
    user_input_policies: HashMap<PolicyId, Policy<P>>,
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
    roles: Option<&'a Vec<&'a Role>>,
    user: Option<&'a User>,
    groups: Option<&'a Vec<&'a Group>>,
    account: &'a AwsAccount,
    target_region: &'a str,
}

impl<'a> Display for PolicyFilterParams<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PolicyFilterParams account: {:?} target_region: {:?} group: {:?}",
            self.account, self.target_region, self.groups
        )
    }
}

impl<'a> PolicyFilterParams<'a> {
    pub const fn new_with(account: &'a AwsAccount, target_region: &'a str) -> Self {
        Self {
            account,
            target_region,
            roles: None,
            user: None,
            groups: None,
        }
    }

    pub const fn roles(mut self, roles: &'a Vec<&'a Role>) -> Self {
        self.roles = Some(roles);
        self
    }

    pub const fn user(mut self, user: &'a User) -> Self {
        self.user = Some(user);
        self
    }

    pub const fn groups(mut self, groups: &'a Vec<&'a Group>) -> Self {
        self.groups = Some(groups);
        self
    }

    pub const fn account(mut self, account: &'a AwsAccount) -> Self {
        self.account = account;
        self
    }

    pub const fn target_region(mut self, target_region: &'a str) -> Self {
        self.target_region = target_region;
        self
    }
}

#[derive(Debug)]
pub struct FoundPolicies<'a, P: Modeled> {
    pub condition: Vec<&'a Policy<ConditionPolicy>>,
    pub user_input: Vec<&'a Policy<P>>,
}

impl<P: Modeled + DeserializeOwned + Send> CoreState<CoreConfig<P>> for IamContainer<P> {
    fn new_from(config: CoreConfig<P>) -> ProxyResult<Self> {
        let accounts = config
            .accounts
            .into_iter()
            .map(|account| (account.code.clone(), account))
            .collect();
        let users = config
            .users
            .clone()
            .into_iter()
            .map(|user| (user.id.clone(), user))
            .collect();
        let groups = config
            .groups
            .into_iter()
            .map(|group| (group.id.clone(), group))
            .collect();
        let user_input_policies = config
            .user_input_policies
            .into_iter()
            .map(|policy| (policy.id.clone(), policy))
            .collect();
        let condition_policies = config
            .condition_policies
            .into_iter()
            .map(|policy| (policy.id.clone(), policy))
            .collect();
        let policy_relationships = config.policy_relationships;

        #[cfg(feature = "prefilter")]
        let prefilter::GroupsContent {
            condition_policies,
            user_group_relationships,
            groups,
            policy_relationships,
            user_input_policies,
        } = prefilter::GroupsContent {
            condition_policies,
            user_group_relationships: config.user_group_relationships,
            groups,
            policy_relationships,
            user_input_policies,
        }
        .filter_by_proxy_condition(Condition::new_with_region_env(
            &PROXY_REGION.load(),
            &PROXY_ENV.load(),
        ));

        let base_access_key_to_user_id = config
            .users
            .into_iter()
            .map(|user| (user.base_access_key, user.id))
            .collect();

        let mut user_id_to_group_ids: HashMap<UserId, Vec<GroupId>> = HashMap::default();
        for rel in user_group_relationships {
            match user_id_to_group_ids.get_mut(&rel.user_id) {
                None => {
                    user_id_to_group_ids.insert(rel.user_id, vec![rel.group_id]);
                }
                Some(group_ids) => {
                    group_ids.push(rel.group_id);
                }
            }
        }

        Ok(Self {
            accounts,
            users,
            groups,
            condition_policies,
            user_input_policies,
            base_access_key_to_user_id,
            user_id_to_group_ids,
            policy_relationships,
        })
    }
}

#[cfg(feature = "prefilter")]
pub mod prefilter {
    use std::collections::HashMap;

    use itertools::Itertools;
    use piam_core::{
        condition::input::Condition,
        group::{Group, GroupId},
        policy::{condition::ConditionPolicy, Modeled, Policy, PolicyId},
        relation_model::{PolicyRelationship, UserGroupRelationship},
    };

    pub struct GroupsContent<P: Modeled> {
        pub condition_policies: HashMap<PolicyId, Policy<ConditionPolicy>>,
        pub user_group_relationships: Vec<UserGroupRelationship>,
        pub groups: HashMap<GroupId, Group>,
        pub policy_relationships: Vec<PolicyRelationship>,
        pub user_input_policies: HashMap<PolicyId, Policy<P>>,
    }

    /// Sometimes it is needed to filter out groups and there related content
    /// that are not applicable to the current proxy condition
    impl<P: Modeled> GroupsContent<P> {
        pub fn filter_by_proxy_condition(self, condition: Condition) -> Self {
            let (keep, drop) = self
                .condition_policies
                .into_iter()
                .partition(|(_, policy)| {
                    policy.modeled_policy.iter().any(|cp| {
                        cp.range
                            .proxy
                            .as_ref()
                            .map_or(true, |r| r.matches(&condition))
                    })
                });
            let condition_policies: HashMap<PolicyId, Policy<ConditionPolicy>> = keep;

            let condition_policy_ids_to_drop: Vec<PolicyId> = drop.clone().into_keys().collect();
            #[allow(clippy::needless_collect)]
            let policy_relationships: Vec<PolicyRelationship> = self
                .policy_relationships
                .into_iter()
                .filter(|r| !condition_policy_ids_to_drop.contains(&r.policy_id))
                .collect();

            let group_ids_to_drop = drop
                .into_iter()
                .flat_map(|(_, policy)| {
                    policy
                        .modeled_policy
                        .into_iter()
                        .filter_map(|cp| cp.range.group_ids)
                        .flatten()
                })
                .unique()
                .collect::<Vec<_>>();
            let user_group_relationships = self
                .user_group_relationships
                .into_iter()
                .filter(|r| !group_ids_to_drop.contains(&r.group_id))
                .collect();
            let groups = self
                .groups
                .into_iter()
                .filter(|(id, _)| !group_ids_to_drop.contains(id))
                .collect();

            let (keep, drop) = policy_relationships.into_iter().partition(|r| {
                r.group_id
                    .as_ref()
                    .map_or(true, |id| !group_ids_to_drop.contains(id))
            });
            let policy_relationships: Vec<PolicyRelationship> = keep;

            let policy_ids_to_drop: Vec<PolicyId> = drop.into_iter().map(|r| r.policy_id).collect();
            let user_input_policies = self
                .user_input_policies
                .into_iter()
                .filter(|(id, _)| !policy_ids_to_drop.contains(id))
                .collect();

            Self {
                condition_policies,
                user_group_relationships,
                groups,
                policy_relationships,
                user_input_policies,
            }
        }
    }
}

impl<P: Modeled> IamContainer<P> {
    pub fn find_account_by_code(&self, code: &str) -> ProxyResult<&AwsAccount> {
        self.accounts.get(code).ok_or_else(|| {
            ProxyError::InvalidAccessKey(format!(
                "Account not found for access key with code: {code}"
            ))
        })
    }

    pub fn find_user_by_base_access_key(&self, base_access_key: &str) -> ProxyResult<&User> {
        let user_id = self
            .base_access_key_to_user_id
            .get(base_access_key)
            .ok_or_else(|| {
                ProxyError::InvalidAccessKey(format!(
                    "User not found for base access key id: '{base_access_key}'"
                ))
            })?;
        self.users
            .get(user_id)
            .ok_or_else(|| ProxyError::UserNotFound(format!("User not found by id: {user_id}")))
    }

    pub fn find_groups_by_user(&self, user: &User) -> ProxyResult<Vec<&Group>> {
        let group_ids = self.user_id_to_group_ids.get(&user.id).ok_or_else(|| {
            ProxyError::GroupNotFound(format!(
                "Groups not found for user id: {}, check proxy_region_env",
                user.id
            ))
        })?;

        group_ids
            .iter()
            .map(|group_id| {
                self.groups.get(group_id).ok_or_else(|| {
                    ProxyError::AssertFail(format!("Group not found by id: {group_id}"))
                })
            })
            .collect()
    }

    pub fn find_policies(&self, f: &PolicyFilterParams) -> ProxyResult<FoundPolicies<P>> {
        let relations: Vec<&PolicyRelationship> = self
            .policy_relationships
            .iter()
            .filter(|r| filter_one(Some(&f.account.id), Some(&r.account_id)))
            .filter(|r| filter_one(Some(f.target_region), Some(&r.region)))
            .filter(|r| {
                let user_id = f.user.map(|u| u.id_str());
                filter_one(user_id, r.user_id.as_deref())
            })
            .filter(|r| {
                let group_ids = f.groups.map(|v| v.iter().map(|g| g.id_str()));
                filter_many(group_ids, r.group_id.as_deref())
            })
            .filter(|r| {
                let role_ids = f.roles.map(|v| v.iter().map(|r| r.id_str()));
                filter_many(role_ids, r.role_id.as_deref())
            })
            .collect();

        if relations.is_empty() {
            return Err(ProxyError::MissingPolicy(format!(
                "access denied by missing policy, PolicyFilterParams: {}",
                f
            )));
        }

        let mut condition: Vec<&Policy<ConditionPolicy>> = Vec::new();
        let mut user_input: Vec<&Policy<P>> = Vec::new();
        for relation in relations {
            match relation.policy_model.as_str() {
                CONDITION => {
                    let p = self.condition_policies.get(&relation.policy_id).unwp();
                    condition.push(p);
                }
                user_input_model if user_input_model == POLICY_MODEL.load().to_string() => {
                    let p = self.user_input_policies.get(&relation.policy_id).unwp();
                    user_input.push(p);
                }
                other => {
                    return Err(ProxyError::AssertFail(format!(
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
}

#[inline]
fn filter_one(query_param: Option<&str>, record: Option<&str>) -> bool {
    query_param.map_or(true, |q| record.map_or(false, |r| equals_or_any(q, r)))
}

#[inline]
fn filter_many<'a>(
    query_params: Option<impl Iterator<Item = &'a str>>,
    record: Option<&str>,
) -> bool {
    query_params.map_or(true, |mut i| {
        record.map_or(false, |r| i.any(|p| equals_or_any(p, r)))
    })
}

#[inline]
fn equals_or_any(query_param: &str, record: &str) -> bool {
    record == query_param || record == ANY
}

#[cfg(test)]
mod test {
    use busylib::ANY;

    use crate::container::{filter_many, filter_one};

    #[clippy::cognitive_complexity = "100"]
    #[test]
    fn test_filter_one() {
        assert!(filter_one(None, None));
        assert!(filter_one(None, Some("val")));
        assert!(filter_one(None, Some(ANY)));

        assert!(filter_one(Some("val"), Some("val")));
        assert!(filter_one(Some("val"), Some(ANY)));
        assert!(filter_one(Some(ANY), Some(ANY)));
        assert!(!filter_one(Some("val"), None));
        assert!(!filter_one(Some("val"), Some("val2")));
    }

    #[clippy::cognitive_complexity = "100"]
    #[test]
    fn test_filter_many() {
        // TODO: satisfy compiler
        // assert!(filter_many(None, None));
        // assert!(filter_many(None, Some("val")));
        // assert!(filter_many(None, Some(ANY)));

        assert!(filter_many(Some(vec!["val"].into_iter()), Some("val")));
        assert!(filter_many(Some(vec!["val"].into_iter()), Some(ANY)));
        assert!(filter_many(Some(vec![ANY].into_iter()), Some(ANY)));
        assert!(!filter_many(Some(vec!["val"].into_iter()), None));
        assert!(!filter_many(Some(vec!["val"].into_iter()), Some("val2")));
        assert!(!filter_many(Some(vec![ANY].into_iter()), Some("val2")));

        assert!(filter_many(
            Some(vec!["val", "val2"].into_iter()),
            Some("val")
        ));
        assert!(filter_many(
            Some(vec!["val", "val2"].into_iter()),
            Some("val2")
        ));
        assert!(filter_many(
            Some(vec!["val", "val2"].into_iter()),
            Some(ANY)
        ));
        assert!(filter_many(Some(vec!["val", ANY].into_iter()), Some("val")));
        assert!(filter_many(Some(vec!["val", ANY].into_iter()), Some(ANY)));
        assert!(!filter_many(
            Some(vec!["val", ANY].into_iter()),
            Some("val2")
        ));
    }
}
