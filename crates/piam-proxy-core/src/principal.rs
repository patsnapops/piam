use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PrincipalContainer {
    pub user_by_access_key: HashMap<String, User>,
    pub group_by_user: HashMap<Uuid, Group>,
}

impl PrincipalContainer {
    pub fn find_user_by_access_key(&self, access_key: &str) -> Option<&User> {
        self.user_by_access_key.get(access_key)
    }

    pub fn find_group_by_user(&self, user: &User) -> Option<&Group> {
        self.group_by_user.get(&user.id)
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    // sample: AKPSTEAMXXX
    pub access_key: String,
    pub secret_key: String,
    pub kind: UserKind,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum UserKind {
    #[default]
    // SVCS
    Service,
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
pub struct Group {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Role {
    pub name: String,
}
