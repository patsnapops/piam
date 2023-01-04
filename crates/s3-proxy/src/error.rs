#![allow(unused)]

use piam_object_storage::error::ParserError;
use piam_proxy::error::ProxyError;

pub type S3ProxyResult<T> = Result<T, S3ProxyError>;

#[derive(Debug)]
pub enum S3ProxyError {}

pub fn from_parser_into_proxy_error(e: ParserError) -> ProxyError {
    ProxyError::ParserError(e.to_string())
}
