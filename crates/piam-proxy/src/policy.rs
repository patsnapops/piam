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
        Ok(self.iter().flat_map(|p| p.find_effects(input)).collect())
    }
}
