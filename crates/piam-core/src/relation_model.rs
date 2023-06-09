//! Relation of IAM domain entities.

use serde::{Deserialize, Serialize};

use crate::{
    account::AccountId,
    group::GroupId,
    policy::PolicyId,
    principal::{RoleId, UserId},
    type_alias::IamEntityIdType,
};

/// n to n
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserGroupRelationship {
    pub id: IamEntityIdType,
    pub user_id: UserId,
    pub group_id: GroupId,
}

/// Policy IDs can be filtered by data_model, group_id, user_id, role_id, account_id and region
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PolicyRelationship {
    pub id: IamEntityIdType,
    /// example: ObjectStorage, DocumentDatabase, MessageQueue.
    pub policy_model: String,
    pub user_id: Option<UserId>,
    pub group_id: Option<GroupId>,
    pub role_id: Option<RoleId>,
    pub account_id: AccountId,
    pub region: String,
    pub policy_id: PolicyId,
}
