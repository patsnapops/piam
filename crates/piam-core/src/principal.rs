use serde::{Deserialize, Serialize};

use crate::{type_alias::IamEntityIdType, IamIdentity};

pub type UserId = IamEntityIdType;
pub type RoleId = IamEntityIdType;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub name: String,
    // example: AKPSTEAMXXX
    pub base_access_key: String,
    pub secret: String,
    pub kind: UserKind,
}

impl IamIdentity for User {
    fn id_str(&self) -> &str {
        &self.id
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum UserKind {
    // SVCS
    Service,
    #[default]
    // PERS
    Person,
    // TEAM
    Team,
    // COMP
    Company,
    // CUST
    Customer,
}

#[derive(Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Role {
    pub id: RoleId,
    pub name: String,
}

impl IamIdentity for Role {
    fn id_str(&self) -> &str {
        &self.id
    }
}
