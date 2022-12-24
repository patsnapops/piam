use serde::de::DeserializeOwned;

use crate::{
    effect::Effect,
    input::Input,
    policy::{Modeled, Policy},
    proxy::error::{ProxyError, ProxyResult},
};

impl<P, I> Policy<P>
where
    P: Modeled<Input = I> + DeserializeOwned,
    I: Input,
{
    fn find_effects(&self, input: &I) -> ProxyResult<Vec<&Effect>> {
        let modeled_policies = &self.modeled_policy;
        if modeled_policies.is_empty() {
            return Err(ProxyError::OtherInternal(format!(
                "policy {} has no model policies",
                self.id
            )));
        }

        let model_effects: Vec<&Effect> = modeled_policies
            .iter()
            .filter_map(|policy| policy.find_effect_by_input(input))
            .collect();
        Ok(model_effects)
    }
}

pub trait FindEffect<P, I>
where
    P: Modeled<Input = I>,
    I: Input,
{
    fn find_effects(&self, input: &I) -> ProxyResult<Vec<&Effect>>;
}

impl<P, I> FindEffect<P, I> for Vec<&Policy<P>>
where
    P: Modeled<Input = I> + DeserializeOwned,
    I: Input,
{
    fn find_effects(&self, input: &I) -> ProxyResult<Vec<&Effect>> {
        let mut all_effects = Vec::new();
        for policy in self {
            all_effects.extend(policy.find_effects(input)?);
        }
        Ok(all_effects)
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
