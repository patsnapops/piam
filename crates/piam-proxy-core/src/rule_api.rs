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

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use uuid::Uuid;

    use crate::{
        policy::{
            s3_policy::{tests::make_data_team_policy, S3PolicyStatement},
            PolicyContainer,
        },
        principal::{Group, PrincipalContainer, User, UserKind},
        rule_api::get_policies,
    };

    pub fn make_one_s3_policy_container() -> PolicyContainer<S3PolicyStatement> {
        let policy_by_group = HashMap::from([(Uuid::default(), vec![make_data_team_policy()])]);

        PolicyContainer {
            policy_by_user: Default::default(),
            policy_by_group,
            policy_by_role: Default::default(),
        }
    }

    fn do_ser_one_s3_policy_container() -> String {
        serde_yaml::to_string(&make_one_s3_policy_container()).unwrap()
    }

    #[test]
    fn ser_one_s3_policy_container() {
        println!("{}", do_ser_one_s3_policy_container());
    }

    #[test]
    fn de_one_s3_policy_container() {
        let s = do_ser_one_s3_policy_container();
        let c: PolicyContainer<S3PolicyStatement> = serde_yaml::from_str(&s).unwrap();
        dbg!(c);
    }

    #[tokio::test]
    async fn _show_get_policies() {
        let container: PolicyContainer<S3PolicyStatement> = get_policies("ObjectStorage").await;
        dbg!(container);
    }
}
