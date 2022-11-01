use std::{collections::HashMap, fmt::Debug};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    effect::Effect,
    principal::{Group, User},
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
}
