use std::{collections::HashMap, fmt::Debug};

use log::debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    condition::ConditionRange,
    effect::Effect,
    error::{ProxyError, ProxyResult},
    input::Input,
    type_alias::IamEntityIdType,
};

pub type PolicyId = IamEntityIdType;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Policy<S: Debug> {
    pub kind: String,
    pub version: i32,
    pub id: PolicyId,
    pub name: String,
    /// if condition specified, only takes effect when condition is met
    pub conditions: Option<Vec<ConditionRange>>,
    pub statements: Vec<S>,
}

pub trait Statement {
    type Input;

    fn version(&self) -> i32;

    fn id(&self) -> String;

    fn find_effect_by_input(&self, input: &Self::Input) -> Option<&Effect>;
}

impl<S, I> Policy<S>
where
    S: Statement<Input = I> + Debug,
    I: Input,
{
    fn find_effects_by_input(&self, input: &I) -> ProxyResult<Vec<&Effect>> {
        // TODO: check condition find_effects_by_input
        let statements = &self.statements;
        if statements.is_empty() {
            return Err(ProxyError::OtherInternal(format!(
                "policy {} has no statements",
                self.id
            )));
        }
        let effects = statements
            .iter()
            .filter_map(|statement| statement.find_effect_by_input(input))
            .collect();
        Ok(effects)
    }
}

pub trait FindEffect<S, I>
where
    S: Statement<Input = I> + Debug,
    I: Input,
{
    fn find_effects_by_input(&self, input: &I) -> ProxyResult<Vec<&Effect>>;
}

impl<S, I> FindEffect<S, I> for Vec<&Policy<S>>
where
    S: Statement<Input = I> + Debug,
    I: Input,
{
    fn find_effects_by_input(&self, input: &I) -> ProxyResult<Vec<&Effect>> {
        let mut all_effects = Vec::new();
        for policy in self {
            all_effects.extend(policy.find_effects_by_input(input)?);
        }
        Ok(all_effects)
    }
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
    pub fn matches(&self, name: &str) -> bool {
        // TODO: static analyze Name
        // should have at least one of eq or start_with
        // should not conflict
        if let Some(eq) = &self.eq {
            if eq.contains(&name.to_string()) {
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

#[cfg(feature = "object-storage-policy")]
pub mod object_storage_policy {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use crate::{effect::Effect, policy::Name};

    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct ObjectStorageStatement {
        pub version: i32,
        pub id: String,
        pub input_statement: ObjectStorageInputStatement,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub output_statement: Option<String>,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct ObjectStorageInputStatement {
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
