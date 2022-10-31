use std::fmt::Debug;

use serde::de::DeserializeOwned;

use crate::{config::get_resource_string, policy::PolicyContainer, principal::PrincipalContainer};

pub async fn get_principals() -> PrincipalContainer {
    let res = get_resource_string("principals").await;
    serde_yaml::from_str(&res).expect("deser principals error")
}

pub async fn get_policies<S: DeserializeOwned + Debug>(kind: &str) -> PolicyContainer<S> {
    let res = get_resource_string(&format!("policies/{}", kind)).await;
    serde_yaml::from_str(&res).expect("deser policies error")
}
