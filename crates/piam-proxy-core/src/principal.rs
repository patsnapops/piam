use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PrincipalContainer {
    pub user_by_access_key: HashMap<String, User>,
    pub group_by_user: HashMap<User, Group>,
}

impl PrincipalContainer {
    pub fn find_user_by_access_key(&self, access_key: &str) -> Option<&User> {
        self.user_by_access_key.get(access_key)
    }

    pub fn find_group_by_user(&self, user: &User) -> Option<&Group> {
        self.group_by_user.get(user)
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

#[cfg(test)]
pub mod test {
    use std::collections::HashMap;

    use uuid::Uuid;

    use crate::principal::{Group, PrincipalContainer, User, UserKind};

    // noinspection SpellCheckingInspection
    pub fn make_principals() -> PrincipalContainer {
        let user_proxy_dev = User {
            id: Uuid::parse_str("51114a7a-8655-4a9e-b2db-1b4124f73e59").unwrap(),
            name: "PROXY DEV".into(),
            access_key: "AKPSSVCSPROXYDEV".into(),
            secret_key: "".into(),
            kind: UserKind::Service,
        };
        let user_lyc = User {
            id: Uuid::parse_str("ca95fbdb-8334-4ee7-a7e7-7350d8921170").unwrap(),
            name: "李元铖".into(),
            access_key: "AKPSPERSLIYCH".into(),
            secret_key: "".into(),
            kind: UserKind::Person,
        };
        let user_dt_svc = User {
            id: Uuid::parse_str("e6244535-10c6-4039-bcd5-aee8747bf370").unwrap(),
            name: "SVCSDATALAKE".into(),
            access_key: "AKPSSVCSDATALAKE".into(),
            secret_key: "".into(),
            kind: UserKind::Service,
        };
        let user_opst = User {
            id: Uuid::parse_str("ff4b1453-f7a6-4ed8-a05a-2cac98046194").unwrap(),
            name: "AKPSSVCSOPST Name".into(),
            access_key: "AKPSSVCSOPST".into(),
            secret_key: "".into(),
            kind: UserKind::Service,
        };
        PrincipalContainer {
            user_by_access_key: HashMap::from([
                ("AKPSSVCSPROXYDEV".to_string(), user_proxy_dev.clone()),
                ("AKPSPERSLIYCH".to_string(), user_lyc.clone()),
                ("AKPSSVCSDATALAKE".to_string(), user_dt_svc.clone()),
                ("AKPSSVCSOPST".to_string(), user_opst.clone()),
            ]),
            group_by_user: HashMap::from([
                (
                    user_proxy_dev,
                    Group {
                        id: Uuid::parse_str("7b3bb7e0-e267-47f1-a689-ec85f25031d5").unwrap(),
                        name: "Proxy DEV Group".into(),
                    },
                ),
                (
                    user_lyc,
                    Group {
                        id: Uuid::parse_str("dad8fea0-e29b-41ba-95e8-7d72b185604e").unwrap(),
                        name: "李元铖 Group".into(),
                    },
                ),
                (
                    user_dt_svc,
                    Group {
                        id: Uuid::parse_str("d8bf9122-1252-49f1-a834-081b18675b2a").unwrap(),
                        name: "Data team Group".into(),
                    },
                ),
                (
                    user_opst,
                    Group {
                        id: Uuid::parse_str("2f0e4efc-911f-48ea-8f90-1483537d422b").unwrap(),
                        name: "OPST Group".into(),
                    },
                ),
            ]),
        }
    }

    #[test]
    fn ser_principals() {
        let string = serde_yaml::to_string(&make_principals()).unwrap();
        println!("{}", string);
    }
}
