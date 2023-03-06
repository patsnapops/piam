use std::fmt;

use piam_core::type_alias::HttpRequest;

use crate::input::ObjectStorageInput;

pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Clone, Debug)]
pub enum ParserError {
    MalformedProtocol(String),
    OperationNotSupported(String),
    InvalidEndpoint(String),
    Internal(String),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MalformedProtocol(msg) => write!(f, "MalformedProtocol: {}", msg),
            Self::OperationNotSupported(msg) => write!(f, "OperationNotSupported: {msg}"),
            Self::InvalidEndpoint(msg) => write!(f, "InvalidEndpoint: {msg}"),
            Self::Internal(msg) => write!(f, "Internal: {msg}"),
        }
    }
}

pub fn parse_error(msg: &str, req: &HttpRequest) -> ParserResult<ObjectStorageInput> {
    let uri = req.uri().to_string();
    let method = req.method().to_string();
    let headers = req.headers().to_owned();
    Err(ParserError::OperationNotSupported(format!(
        "{msg}, supported apis: \
        http://ida.patsnap.info/piam/docs/user/s3/feat#%E6%94%AF%E6%8C%81%E7%9A%84-api \
        request: uri: {uri} method: {method} headers: {headers:?} "
    )))
}
