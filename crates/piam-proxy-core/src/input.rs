use std::fmt::Debug;

use anyhow::Result;

use crate::type_alias::HttpRequest;

pub trait Input: Sized + Debug {
    fn from_http(req: &HttpRequest) -> Result<Self>;
}
