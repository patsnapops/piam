use piam_core::{
    effect::Effect,
    input::Input,
    policy::{Modeled, Policy},
};
use serde::de::DeserializeOwned;

use crate::error::ProxyResult;

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
            all_effects.extend(policy.find_effects(input));
        }
        Ok(all_effects)
    }
}
