use std::fmt::Debug;

use crate::{config::ParserConfig, error::ProxyResult, type_alias::HttpRequest};

pub trait Input: Sized + Debug {
    type Config: ParserConfig;

    /// Some type of request needs config to be parsed, such as s3 request, which needs host.
    fn from_http(req: &HttpRequest, config: Option<&Self::Config>) -> ProxyResult<Self>;
}
