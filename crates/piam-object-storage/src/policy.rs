use piam_core::{
    effect::Effect,
    policy::{Modeled, Name},
};
use serde::{Deserialize, Serialize};

use crate::input::{ActionKind, ObjectStorageInput};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ObjectStoragePolicy {
    pub version: i32,
    pub id: String,
    pub input_policy: ObjectStorageInputPolicy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_policy: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ObjectStorageInputPolicy {
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

impl Modeled for ObjectStoragePolicy {
    type Input = ObjectStorageInput;

    fn version(&self) -> i32 {
        self.version
    }

    fn id(&self) -> String {
        self.id.clone()
    }

    fn find_effect_by_input(&self, input: &Self::Input) -> Option<&Effect> {
        let input_policy = &self.input_policy;
        if !input_policy.match_action(input) {
            return None;
        }
        match input.action_kind() {
            ActionKind::ListBuckets | ActionKind::Bucket => input_policy.find_bucket_effect(input),
            ActionKind::Object => input_policy.find_object_effect(input),
        }
    }
}

/// Modeling for ObjectStoragePolicy
trait ObjectStorageMatches {
    fn match_action(&self, input: &ObjectStorageInput) -> bool;
    fn find_bucket_effect(&self, input: &ObjectStorageInput) -> Option<&Effect>;
    fn find_object_effect(&self, input: &ObjectStorageInput) -> Option<&Effect>;
}

impl ObjectStorageMatches for ObjectStorageInputPolicy {
    fn match_action(&self, input: &ObjectStorageInput) -> bool {
        match &self.actions {
            None => true,
            Some(actions) => {
                actions.contains(&input.action()) || actions.contains(&"Any".to_string())
            }
        }
    }

    fn find_bucket_effect(&self, input: &ObjectStorageInput) -> Option<&Effect> {
        let bucket = &self.bucket;
        match &bucket.name {
            None => bucket.effect.as_ref(),
            Some(name) => match name.matches(input.bucket()) {
                true => bucket.effect.as_ref(),
                false => None,
            },
        }
    }

    fn find_object_effect(&self, input: &ObjectStorageInput) -> Option<&Effect> {
        // TODO: reduce indentation
        self.find_bucket_effect(input)?;
        match &self.bucket.keys {
            None => None,
            Some(keys) => {
                if keys.is_empty() {
                    return None;
                }
                let mut default_for_all_name: Option<&Effect> = None;
                for key in keys {
                    match &key.name {
                        Some(name) => {
                            if name.matches(input.key()) {
                                return key.effect.as_ref();
                            }
                        }
                        None => {
                            // TODO: static analyze, this condition can occur at most once at runtime
                            default_for_all_name = key.effect.as_ref();
                        }
                    }
                }
                default_for_all_name
            }
        }
    }
}

#[cfg(test)]
mod test {
    use piam_core::{
        effect::{Effect, Modify},
        policy::Name,
    };

    use crate::{
        input::ObjectStorageInput,
        policy::{Key, ObjectStorageInputPolicy, ObjectStorageMatches},
    };

    #[test]
    fn match_action() {
        let policy = ObjectStorageInputPolicy {
            actions: Some(vec!["CreateBucket".to_string(), "GetObject".to_string()]),
            ..Default::default()
        };

        let list_buckets = ObjectStorageInput::ListBuckets;
        let create_bucket = ObjectStorageInput::CreateBucket {
            bucket: "bucket".to_string(),
        };
        let head_bucket = ObjectStorageInput::HeadBucket {
            bucket: "bucket".to_string(),
        };
        let get_object = ObjectStorageInput::GetObject {
            bucket: "bucket".to_string(),
            key: "key".to_string(),
        };
        let put_object = ObjectStorageInput::PutObject {
            bucket: "bucket".to_string(),
            key: "key".to_string(),
        };

        assert!(!policy.match_action(&list_buckets));
        assert!(policy.match_action(&create_bucket));
        assert!(!policy.match_action(&head_bucket));

        assert!(policy.match_action(&get_object));
        assert!(!policy.match_action(&put_object));
    }

    #[test]
    fn match_bucket_effect() {
        let mut policy = ObjectStorageInputPolicy::default();
        policy.bucket.name = Some(Name {
            eq: Some(vec![String::from("bucket1")]),
            start_with: Some(vec![String::from("start")]),
        });
        policy.bucket.effect = Some(Effect::allow());

        let create_bucket_1 = ObjectStorageInput::CreateBucket {
            bucket: "bucket1".to_string(),
        };
        let create_bucket_2 = ObjectStorageInput::CreateBucket {
            bucket: "bucket2".to_string(),
        };
        let create_bucket_3 = ObjectStorageInput::CreateBucket {
            bucket: "start_bucket".to_string(),
        };

        assert!(policy.find_bucket_effect(&create_bucket_1).is_some());
        assert!(policy.find_bucket_effect(&create_bucket_2).is_none());
        assert!(policy.find_bucket_effect(&create_bucket_3).is_some());

        policy.bucket.name = None;
        assert!(policy.find_bucket_effect(&create_bucket_1).is_some());
    }

    #[test]
    fn match_object_effect() {
        let mut policy = ObjectStorageInputPolicy::default();
        policy.bucket.name = Some(Name {
            eq: Some(vec![String::from("bucket1")]),
            start_with: Some(vec![String::from("start1")]),
        });
        let bucket_effect = Effect::Allow {
            emit_event: None,
            rate_limit: None,
            modify: None,
        };
        policy.bucket.effect = Some(bucket_effect);

        let key_effect_1 = Effect::Allow {
            emit_event: None,
            rate_limit: None,
            modify: Some(Modify {}),
        };
        let key1 = Key {
            name: Some(Name {
                eq: Some(vec![String::from("key1")]),
                start_with: Some(vec![String::from("start2")]),
            }),
            effect: Some(key_effect_1.clone()),
            ..Default::default()
        };
        let key_effect_2 = Effect::Deny(None);
        let key2 = Key {
            name: Some(Name {
                eq: Some(vec![String::from("key2")]),
                start_with: Some(vec![String::from("start3")]),
            }),
            effect: Some(key_effect_2.clone()),
            ..Default::default()
        };
        policy.bucket.keys = Some(vec![key1, key2]);

        let get_object_1 = ObjectStorageInput::GetObject {
            bucket: "bucket1".to_string(),
            key: "key1".to_string(),
        };
        let get_object_2 = ObjectStorageInput::GetObject {
            bucket: "bucket1".to_string(),
            key: "key2".to_string(),
        };
        let get_object_3 = ObjectStorageInput::GetObject {
            bucket: "bucket1".to_string(),
            key: "key3".to_string(),
        };
        let get_object_4 = ObjectStorageInput::GetObject {
            bucket: "bucket2".to_string(),
            key: "key1".to_string(),
        };

        assert_eq!(
            policy.find_object_effect(&get_object_1),
            Some(&key_effect_1)
        );
        assert_eq!(
            policy.find_object_effect(&get_object_2),
            Some(&key_effect_2)
        );
        assert_eq!(policy.find_object_effect(&get_object_3), None);
        assert_eq!(policy.find_object_effect(&get_object_4), None);

        policy.bucket.keys = None;
        assert_eq!(policy.find_object_effect(&get_object_1), None);

        policy.bucket.keys = Some(vec![
            Key {
                name: None,
                tag: None,
                effect: Some(Effect::allow()),
            },
            Key {
                name: Some(Name {
                    eq: Some(vec!["key2".to_string()]),
                    start_with: None,
                }),
                tag: None,
                effect: Some(Effect::deny()),
            },
        ]);
        assert_eq!(
            policy.find_object_effect(&get_object_1),
            Some(&Effect::allow())
        );
        assert_eq!(
            policy.find_object_effect(&get_object_2),
            Some(&Effect::deny())
        );

        policy.bucket.keys = Some(vec![
            Key {
                name: None,
                tag: None,
                effect: Some(Effect::deny()),
            },
            Key {
                name: Some(Name {
                    eq: Some(vec!["key2".to_string()]),
                    start_with: None,
                }),
                tag: None,
                effect: Some(Effect::allow()),
            },
        ]);
        assert_eq!(
            policy.find_object_effect(&get_object_1),
            Some(&Effect::deny())
        );
        assert_eq!(
            policy.find_object_effect(&get_object_2),
            Some(&Effect::allow())
        );

        policy.bucket.keys = Some(vec![]);
        assert_eq!(policy.find_object_effect(&get_object_1), None);
    }
}
