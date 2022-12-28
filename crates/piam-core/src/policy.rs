//! Policy is an abstraction of a resource model specific policy such as `ObjectStoragePolicy`.

use std::fmt::Debug;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{effect::Effect, input::Input, type_alias::IamEntityIdType, IamIdentity};

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

impl<P: Modeled> IamIdentity for Policy<P> {
    fn id_str(&self) -> &str {
        &self.id
    }
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

    use busylib::prelude::EnhancedUnwrap;
    use cidr::{AnyIpCidr, Ipv4Cidr};
    use serde::{Deserialize, Serialize};

    use crate::{
        condition::input::{Condition, ConditionCtx},
        effect::Effect,
        group::GroupId,
        policy::Modeled,
    };

    /// If ConditionPolicy is not specified, this phase of effect finding should be skipped.
    /// If ConditionPolicy is specified, only takes its effect when condition is matched.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct ConditionPolicy {
        pub version: i32,
        pub id: String,
        pub range: ConditionRange,
        pub effect: Effect,
    }

    /// If the optional Range is [`None`], it means any Condition matches
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct ConditionRange {
        pub group_ids: Option<Vec<GroupId>>,
        pub from: Option<Range>,
        pub proxy: Option<Range>,
        pub to: Option<Range>,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Range {
        pub ip_cidr: Option<Vec<AnyIpCidr>>,
        pub region: Option<Vec<String>>,
        pub env: Option<Vec<String>>,
    }

    impl ConditionPolicy {
        pub fn find_effect(&self, condition_ctx: &ConditionCtx) -> Option<&Effect> {
            match self.range.matches(condition_ctx) {
                false => None,
                true => Some(&self.effect),
            }
        }
    }

    impl Modeled for ConditionPolicy {
        type Input = ConditionCtx;

        fn version(&self) -> i32 {
            self.version
        }

        fn id(&self) -> String {
            self.id.clone()
        }

        fn find_effect_by_input(&self, condition_ctx: &Self::Input) -> Option<&Effect> {
            match self.range.matches(condition_ctx) {
                false => None,
                true => Some(&self.effect),
            }
        }
    }

    impl ConditionRange {
        pub fn matches(&self, condition_ctx: &ConditionCtx) -> bool {
            let from_matched = match &self.from {
                None => true,
                Some(range) => range.matches(&condition_ctx.from),
            };
            let proxy_matched = match &self.proxy {
                None => true,
                Some(range) => range.matches(&condition_ctx.proxy),
            };
            let to_matched = match &self.to {
                None => true,
                Some(range) => range.matches(&condition_ctx.to),
            };
            from_matched && proxy_matched && to_matched
        }
    }

    impl Range {
        pub fn matches(&self, condition: &Condition) -> bool {
            let ip_cidr_matched = match &self.ip_cidr {
                None => true,
                Some(vec) => match condition.addr {
                    None => false,
                    Some(addr) => vec.iter().any(|cidr| cidr.contains(&addr.ip())),
                },
            };
            let region_matched = match &self.region {
                None => true,
                Some(vec) => match &condition.region {
                    None => false,
                    Some(region) => vec.contains(region),
                },
            };
            let env_matched = match &self.env {
                None => true,
                Some(vec) => match &condition.env {
                    None => false,
                    Some(env) => vec.contains(env),
                },
            };
            ip_cidr_matched && region_matched && env_matched
        }
    }

    pub fn private_ip_cidr() -> Vec<AnyIpCidr> {
        vec![
            AnyIpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(10, 0, 0, 0), 8).unwp()),
            AnyIpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(172, 16, 0, 0), 12).unwp()),
            AnyIpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(192, 168, 0, 0), 16).unwp()),
            AnyIpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(127, 0, 0, 1), 32).unwp()),
        ]
    }

    #[cfg(test)]
    mod test {
        #[test]
        fn condition_range_contains() {
            let vec = &vec!["a".to_string(), "b".to_string()];
            let val = &"b".to_string();
            assert!(vec.contains(val))
        }
    }
}
