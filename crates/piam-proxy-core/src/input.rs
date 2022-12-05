use std::fmt::Debug;

use crate::{error::ProxyResult, type_alias::HttpRequest};

pub trait Input: Sized + Debug {
    type State;

    /// Some type of request needs state to be parsed, such as s3 request, which needs host.
    fn from_http(req: &HttpRequest, state: Option<&Self::State>) -> ProxyResult<Self>;
}
