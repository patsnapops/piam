use std::{collections::HashMap, fmt::Debug};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    effect::Effect,
    principal::{Group, Role, User},
};

pub type UserByAccessKey = HashMap<String, User>;
pub type GroupByUser = HashMap<User, Group>;

/// There can only be one policy takes effect for each request
pub type Policies<S> = Vec<Policy<S>>;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PolicyContainer<S: Debug> {
    pub policy_by_user: HashMap<Uuid, Policies<S>>,
    pub policy_by_group: HashMap<Uuid, Policies<S>>,
    pub policy_by_role: HashMap<Uuid, Policies<S>>,
}

impl<S: Statement + Debug> PolicyContainer<S> {
    pub fn find_policies_by_group(&self, group: &Group) -> Option<&Policies<S>> {
        self.policy_by_group.get(&group.id)
    }
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Policy<S: Debug> {
    pub kind: String,
    pub version: i32,
    pub id: Uuid,
    /// if condition specified, only takes effect when condition is met
    // pub conditions: Vec<Conditions>,
    pub statement: S,
}

pub trait Statement {
    type Input;

    fn version(&self) -> i32;

    fn id(&self) -> String;

    fn find_effect_for_input(&self, input: &Self::Input) -> Option<&Effect>;
}

/// Default logical operator would be `or`. Any name matching `eq`,
/// `start_with`, `contains` will be regarded as a successful match.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Name {
    /// The list used to do the `Eq` match for the given name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eq: Option<Vec<String>>,
    /// The list used to do the `start_with` match for the given name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_with: Option<Vec<String>>,
}

impl Name {
    pub fn matches(&self, name: &String) -> bool {
        // TODO: static analyze Name
        // should have at least one of eq or start_with
        // should not conflict
        if let Some(eq) = &self.eq {
            if eq.contains(name) {
                return true;
            }
        }
        if let Some(start_with) = &self.start_with {
            if start_with.iter().any(|prefix| name.starts_with(prefix)) {
                return true;
            }
        }
        false
    }
}

#[cfg(feature = "s3-policy")]
pub mod s3_policy {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use crate::{effect::Effect, policy::Name};

    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct S3PolicyStatement {
        pub version: i32,
        pub id: Uuid,
        pub input_policy: S3InputPolicyStatement,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub output_policy: Option<String>,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct S3InputPolicyStatement {
        // TODO: use enum ActionName instead of String
        #[serde(skip_serializing_if = "Option::is_none")]
        pub actions: Option<Vec<String>>,
        pub bucket: Bucket,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub control: Option<Control>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub outpost: Option<Outpost>,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Tag {
        pub key_eq: Option<Vec<String>>,
    }

    /// Default logical operator would be `or`. Any bucket name or tag matching
    /// their corresponding field (`name`, `tag`) will trigger the execution
    /// of the `effect`
    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct Bucket {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<Name>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tag: Option<Tag>,
        #[serde(flatten)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub effect: Option<Effect>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub keys: Option<Vec<Key>>,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct Outpost;

    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct Control;

    /// Default logical operator would be `or`, Same as Bucket.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Key {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<Name>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tag: Option<Tag>,
        #[serde(flatten)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub effect: Option<Effect>,
    }

    #[cfg(test)]
    pub mod tests {
        use std::collections::HashMap;
        use std::time::Duration;

        use crate::{
            effect::{Effect, EmitEvent, Log, Metric, RateLimit},
            policy::{
                s3_policy::{Bucket, Key, S3InputPolicyStatement, S3PolicyStatement},
                Name, Policy,
            },
        };
        use crate::policy::{Policies, PolicyContainer};

        pub fn make_one_policy() -> Policy<S3PolicyStatement> {
            let actions = Some(vec![
                "ListObjects".into(),
                "GetObject".into(),
                "PutObject".into(),
            ]);
            let bucket_name = Name {
                eq: Some(vec!["datalake-internal.patsnap.com".into(), "b".into()]),
                start_with: Some(vec!["c".into(), "d".into()]),
            };
            let key_name = Name {
                eq: Some(vec!["ka".into(), "kb".into()]),
                start_with: Some(vec!["kc".into()]),
            };
            let key_name2 = Name {
                eq: Some(vec!["k2a".into(), "k2b".into()]),
                start_with: Some(vec!["k2c".into()]),
            };
            let bucket_effect = Effect::Allow {
                emit_event: None,
                rate_limit: Some(RateLimit {
                    duration: Duration::from_secs(1),
                    count: 2,
                }),
                modify: None,
            };
            let key_effect = Effect::Deny(Some(EmitEvent {
                log: Some(Log {
                    address: "kl".to_string(),
                }),
                metric: Some(Metric {
                    address: "km".to_string(),
                }),
            }));
            let key = Key {
                name: Some(key_name),
                tag: None,
                effect: Some(key_effect),
            };
            let key2 = Key {
                name: Some(key_name2),
                ..Default::default()
            };
            let bucket = Bucket {
                name: Some(bucket_name),
                tag: None,
                effect: Some(bucket_effect),
                keys: Some(vec![key, key2]),
            };
            let input_statement = S3InputPolicyStatement {
                actions,
                bucket,
                ..Default::default()
            };
            let statement = S3PolicyStatement {
                version: 0,
                id: Default::default(),
                input_policy: input_statement,
                output_policy: None,
            };

            Policy {
                kind: "ObjectStorage".to_string(),
                version: 0,
                id: Default::default(),
                statement,
            }
        }

        pub fn make_data_team_policy() -> Policy<S3PolicyStatement> {
            let actions = Some(vec![
                "ListObjects".into(),
                "GetObject".into(),
                "PutObject".into(),
            ]);
            let bucket = Bucket {
                name: Some(Name {
                    eq: Some(vec!["datalake-internal.patsnap.com".into()]),
                    start_with: None,
                }),
                tag: None,
                effect: Some(Effect::allow()),
                keys: Some(vec![Key {
                    name: None,
                    tag: None,
                    effect: Some(Effect::allow()),
                }]),
            };
            let input_statement = S3InputPolicyStatement {
                actions,
                bucket,
                ..Default::default()
            };
            let statement = S3PolicyStatement {
                version: 0,
                id: Default::default(),
                input_policy: input_statement,
                output_policy: None,
            };
            Policy {
                kind: "ObjectStorage".to_string(),
                version: 0,
                id: Default::default(),
                statement,
            }
        }

        fn do_ser_one() -> String {
            let policy = make_data_team_policy();
            dbg!(std::mem::size_of_val(&policy));
            serde_yaml::to_string(&policy).unwrap()
        }

        #[test]
        fn ser_one() {
            dbg!(do_ser_one());
        }

        #[test]
        fn de_one() {
            let statement: Policy<S3PolicyStatement> = serde_yaml::from_str(&do_ser_one()).unwrap();
            dbg!(statement);
        }

        pub fn make_dev_policies() -> Policies<S3PolicyStatement> {
            let actions = Some(vec!["Any".into()]);
            let key = Key {
                name: None,
                tag: None,
                effect: Some(Effect::allow()),
            };
            let bucket = Bucket {
                name: None,
                tag: None,
                effect: Some(Effect::allow()),
                keys: Some(vec![key]),
            };
            let statement = S3PolicyStatement {
                version: 0,
                id: Default::default(),
                input_policy: S3InputPolicyStatement {
                    actions,
                    bucket,
                    ..Default::default()
                },
                output_policy: None,
            };
            let policy = Policy {
                kind: "ObjectStorage".to_string(),
                version: 0,
                id: Default::default(),
                statement,
            };
            vec![policy]
        }

        pub fn make_lyc_policies() -> Policies<S3PolicyStatement> {
            let actions = Some(vec!["ListObjects".into(), "GetObject".into()]);
            let key = Key {
                name: Some(Name {
                    eq: None,
                    start_with: Some(vec!["liych".into()]),
                }),
                tag: None,
                effect: Some(Effect::allow()),
            };
            let bucket = Bucket {
                name: Some(Name {
                    eq: Some(vec!["testpatsnapus".into()]),
                    start_with: None,
                }),
                tag: None,
                effect: Some(Effect::allow()),
                keys: Some(vec![key]),
            };
            let statement = S3PolicyStatement {
                version: 0,
                id: Default::default(),
                input_policy: S3InputPolicyStatement {
                    actions,
                    bucket,
                    ..Default::default()
                },
                output_policy: None,
            };
            let policy = Policy {
                kind: "ObjectStorage".to_string(),
                version: 0,
                id: Default::default(),
                statement,
            };
            vec![policy]
        }

        pub fn make_data_team_svcs_policies() -> Policies<S3PolicyStatement> {
            let actions = Some(vec![
                "ListObjects".into(),
                "GetObject".into(),
                "PutObject".into(),
            ]);
            let key = Key {
                name: None,
                tag: None,
                effect: Some(Effect::allow()),
            };
            let bucket = Bucket {
                name: Some(Name {
                    eq: Some(vec!["datalake-internal.patsnap.com".into()]),
                    start_with: None,
                }),
                tag: None,
                effect: Some(Effect::allow()),
                keys: Some(vec![key]),
            };
            let statement = S3PolicyStatement {
                version: 0,
                id: Default::default(),
                input_policy: S3InputPolicyStatement {
                    actions,
                    bucket,
                    ..Default::default()
                },
                output_policy: None,
            };
            let policy = Policy {
                kind: "ObjectStorage".to_string(),
                version: 0,
                id: Default::default(),
                statement,
            };
            vec![policy]
        }

        // noinspection SpellCheckingInspection
        pub fn make_opst_policies() -> Policies<S3PolicyStatement> {
            let actions = Some(vec![
                "ListObjects".into(),
                "GetObject".into(),
                "PutObject".into(),
                "GetBucketNotificationConfiguration".into(),
                "PutBucketNotificationConfiguration".into(),
            ]);
            let key = Key {
                name: None,
                tag: None,
                effect: Some(Effect::allow()),
            };
            let bucket = Bucket {
                name: None,
                tag: None,
                effect: Some(Effect::allow()),
                keys: Some(vec![key]),
            };
            let statement = S3PolicyStatement {
                version: 0,
                id: Default::default(),
                input_policy: S3InputPolicyStatement {
                    actions,
                    bucket,
                    ..Default::default()
                },
                output_policy: None,
            };
            let policy = Policy {
                kind: "ObjectStorage".to_string(),
                version: 0,
                id: Default::default(),
                statement,
            };
            vec![policy]
        }

        #[test]
        fn ser_lyc_policy() {
            let string = serde_yaml::to_string(&make_lyc_policies()).unwrap();
            println!("{}", string);
        }

        pub fn make_s3_policies() -> PolicyContainer<S3PolicyStatement> {
            use crate::principal::test::*;
            let policy_by_group = HashMap::from([
                (make_dev_group().id, make_dev_policies()),
                (make_lyc_group().id, make_lyc_policies()),
                (make_data_team_svcs_group().id, make_data_team_svcs_policies()),
                (make_opst_group().id, make_opst_policies())
            ]);
            PolicyContainer {
                policy_by_user: Default::default(),
                policy_by_group,
                policy_by_role: Default::default()
            }
        }

        #[test]
        fn ser_s3_policies() {
            let string = serde_yaml::to_string(&make_s3_policies()).unwrap();
            println!("{}", string);
        }
    }
}
