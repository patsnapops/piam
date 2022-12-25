//! Policy is an abstraction of a resource model specific policy such as `ObjectStoragePolicy`.

use std::fmt::Debug;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

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

impl<P, I> Policy<P>
where
    P: Modeled<Input = I> + DeserializeOwned,
    I: Input,
{
    #[allow(dead_code)]
    pub fn find_effects(&self, input: &I) -> Vec<&Effect> {
        self.modeled_policy
            .iter()
            .filter_map(|policy| policy.find_effect_by_input(input))
            .collect()
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

pub mod condition {
    use std::net::Ipv4Addr;

    use cidr::{AnyIpCidr, Ipv4Cidr};
    use serde::{Deserialize, Serialize};

    use crate::{condition::input::Condition, effect::Effect, policy::Modeled};

    /// If ConditionPolicy is not specified, this phase of effect finding should be skipped.
    /// If ConditionPolicy is specified, only takes its effect when condition is matched.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct ConditionPolicy {
        pub version: i32,
        pub id: String,
        pub range: ConditionRange,
        pub effect: Effect,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct ConditionRange {
        pub ip_cidr_range: Option<Vec<AnyIpCidr>>,
        pub region_range: Option<Vec<String>>,
    }

    impl ConditionPolicy {
        pub fn find_effect(&self, condition: &Condition) -> Option<&Effect> {
            match self.range.contains(condition) {
                false => None,
                true => Some(&self.effect),
            }
        }
    }

    impl Modeled for ConditionPolicy {
        type Input = Condition;

        fn version(&self) -> i32 {
            self.version
        }

        fn id(&self) -> String {
            self.id.clone()
        }

        fn find_effect_by_input(&self, condition: &Self::Input) -> Option<&Effect> {
            match self.range.contains(condition) {
                false => None,
                true => Some(&self.effect),
            }
        }
    }

    impl ConditionRange {
        pub fn contains(&self, condition: &Condition) -> bool {
            match &self.ip_cidr_range {
                None => false,
                Some(cidr_vec) => match &condition.addr {
                    None => false, // no addr in Condition which normally come with request
                    Some(addr) => cidr_vec.iter().any(|ip_cidr| ip_cidr.contains(&addr.ip())),
                },
            }
        }

        pub fn private_ip_cidr() -> Vec<AnyIpCidr> {
            vec![
                AnyIpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(10, 0, 0, 0), 8).unwrap()),
                AnyIpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(172, 16, 0, 0), 12).unwrap()),
                AnyIpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(192, 168, 0, 0), 16).unwrap()),
                AnyIpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(127, 0, 0, 1), 32).unwrap()),
            ]
        }
    }

    pub mod test {
        #[test]
        fn condition_range_contains() {
            // TODO: test condition_range_contains
        }
    }
}
