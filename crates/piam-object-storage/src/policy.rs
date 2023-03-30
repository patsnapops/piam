use piam_core::{
    effect::Effect,
    policy::{Modeled, StringMatcher},
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
    /// There can only be one item in keys
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<Key>>,
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
    pub name: Option<StringMatcher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Tag>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<Effect>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Outpost;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Control;

/// Default logical operator would be `or`, Same as Bucket.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Key {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<StringMatcher>,
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
        self.find_bucket_effect(input)?;
        match &self.keys {
            None => None,
            Some(keys) => {
                if keys.is_empty() {
                    return None;
                }
                self.find_keys_effect(input, keys)
            }
        }
    }
}

impl ObjectStorageInputPolicy {
    /// find the first key policy that matches the input, return the effect
    /// assume that key policy in keys are not conflicting
    fn find_keys_effect<'a>(
        &'a self,
        input: &ObjectStorageInput,
        policies: &'a [Key],
    ) -> Option<&Effect> {
        // TODO: static analysis to make sure that key policy in keys are not conflicting
        let mut default_effect = None;
        for policy in policies {
            if let Some(path) = &policy.path {
                if let ObjectStorageInput::DeleteObjects { bucket, keys } = input {
                    if Self::match_delete_objects(bucket, keys, path) {
                        return policy.effect.as_ref();
                    }
                } else if path.matches(&Self::full_path(input.bucket(), input.key())) {
                    return policy.effect.as_ref();
                }
            } else {
                default_effect = policy.effect.as_ref();
            }
        }
        default_effect
    }

    fn match_delete_objects(bucket: &str, keys: &[String], matcher: &StringMatcher) -> bool {
        keys.iter()
            .map(|key| Self::full_path(bucket, key))
            .all(|full_path| matcher.matches(&full_path))
    }

    fn full_path(bucket: &str, key: &str) -> String {
        format!("{}/{}", bucket, key)
    }
}

#[cfg(test)]
mod test {
    use piam_core::{
        effect::{Effect, Modify},
        policy::StringMatcher,
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
        policy.bucket.name = Some(StringMatcher {
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
        policy.bucket.name = Some(StringMatcher {
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
            path: Some(StringMatcher {
                eq: Some(vec![String::from("bucket1/key1")]),
                start_with: Some(vec![String::from("bucket1/start2")]),
            }),
            effect: Some(key_effect_1.clone()),
            ..Default::default()
        };
        let key_effect_2 = Effect::Deny(None);
        let key2 = Key {
            path: Some(StringMatcher {
                eq: Some(vec![String::from("bucket1/key2")]),
                start_with: Some(vec![String::from("start3")]),
            }),
            effect: Some(key_effect_2.clone()),
            ..Default::default()
        };
        policy.keys = Some(vec![key1, key2]);

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

        assert_eq!(
            policy.find_object_effect(&get_object_1),
            Some(&key_effect_1)
        );
        assert_eq!(
            policy.find_object_effect(&get_object_2),
            Some(&key_effect_2)
        );
        assert_eq!(policy.find_object_effect(&get_object_3), None);

        policy.keys = None;
        assert_eq!(policy.find_object_effect(&get_object_1), None);

        policy.keys = Some(vec![
            Key {
                path: None,
                tag: None,
                effect: Some(Effect::allow()),
            },
            Key {
                path: Some(StringMatcher {
                    eq: Some(vec!["bucket1/key2".to_string()]),
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

        policy.keys = Some(vec![
            Key {
                path: None,
                tag: None,
                effect: Some(Effect::deny()),
            },
            Key {
                path: Some(StringMatcher {
                    eq: Some(vec!["bucket1/key2".to_string()]),
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

        policy.keys = Some(vec![]);
        assert_eq!(policy.find_object_effect(&get_object_1), None);
    }

    #[test]
    fn match_delete_objects_effect() {
        let mut policy = ObjectStorageInputPolicy::default();
        policy.bucket.name = Some(StringMatcher {
            eq: Some(vec![String::from("bucket1")]),
            start_with: None,
        });
        policy.bucket.effect = Some(Effect::allow());

        let allow = Effect::allow();
        let deny = Effect::deny();
        let key1 = Key {
            path: Some(StringMatcher {
                eq: None,
                start_with: Some(vec![String::from("bucket1/start1")]),
            }),
            effect: Some(allow.clone()),
            ..Default::default()
        };
        let key2 = Key {
            path: Some(StringMatcher {
                eq: None,
                start_with: Some(vec![String::from("bucket1/start2")]),
            }),
            effect: Some(deny.clone()),
            ..Default::default()
        };

        policy.keys = Some(vec![key1, key2]);
        assert_eq!(
            policy.find_object_effect(&ObjectStorageInput::DeleteObjects {
                bucket: "bucket1".to_string(),
                keys: vec!["start1".to_string()],
            }),
            Some(&allow)
        );
        assert_eq!(
            policy.find_object_effect(&ObjectStorageInput::DeleteObjects {
                bucket: "bucket1".to_string(),
                keys: vec!["start1/".to_string(), "start1/foo".to_string()],
            }),
            Some(&allow)
        );
        assert_eq!(
            policy.find_object_effect(&ObjectStorageInput::DeleteObjects {
                bucket: "bucket2".to_string(),
                keys: vec!["start1".to_string()],
            }),
            None
        );
        assert_eq!(
            policy.find_object_effect(&ObjectStorageInput::DeleteObjects {
                bucket: "bucket1".to_string(),
                keys: vec![
                    "key_not_in_policy".to_string(),
                    "key2_not_in_policy".to_string()
                ],
            }),
            None
        );
        assert_eq!(
            policy.find_object_effect(&ObjectStorageInput::DeleteObjects {
                bucket: "bucket1".to_string(),
                keys: vec!["start1".to_string(), "key_not_in_policy".to_string()],
            }),
            None
        );
        assert_eq!(
            policy.find_object_effect(&ObjectStorageInput::DeleteObjects {
                bucket: "bucket1".to_string(),
                keys: vec!["start2".to_string()],
            }),
            Some(&deny)
        );

        assert_eq!(
            policy.find_object_effect(&ObjectStorageInput::DeleteObjects {
                bucket: "bucket1".to_string(),
                keys: vec!["start2".to_string()],
            }),
            Some(&deny)
        );
        assert_eq!(
            policy.find_object_effect(&ObjectStorageInput::DeleteObjects {
                bucket: "bucket1".to_string(),
                keys: vec!["start2/foo".to_string()],
            }),
            Some(&deny)
        );
        assert_eq!(
            policy.find_object_effect(&ObjectStorageInput::DeleteObjects {
                bucket: "bucket1".to_string(),
                keys: vec!["start2".to_string(), "not_in_policy".to_string()],
            }),
            None
        );
    }
}
