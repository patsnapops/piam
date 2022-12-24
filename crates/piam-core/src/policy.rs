//! Policy is an abstraction of a resource model specific policy such as `ObjectStoragePolicy`.

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::{effect::Effect, input::Input, type_alias::IamEntityIdType};

pub type PolicyId = IamEntityIdType;

/// The policy to be applied to the request. See `Input`
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Policy<P: Modeled> {
    pub kind: String,
    pub version: i32,
    pub id: PolicyId,
    pub name: String,
    /// A generic structure defined by user of this library
    pub modeled_policy: Vec<P>,
}

/// Every kind of policy should be modeled and the `Effect` can be searched from within it.
pub trait Modeled: Debug {
    type Input: Input;

    fn version(&self) -> i32;

    fn id(&self) -> String;

    fn find_effect_by_input(&self, input: &Self::Input) -> Option<&Effect>;
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
