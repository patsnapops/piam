use std::{fmt, fmt::Display};

pub type ProxyResult<T> = Result<T, ProxyError>;

#[derive(Debug)]
pub enum ProxyError {
    BadRequest(String),
    MalformedProtocol(String),
    InvalidEndpoint(String),
    InvalidRegion(String),
    InvalidAuthorizationHeader(String),
    InvalidAccessKey(String),
    ParserError(String),
    OperationNotSupported(String),
    ResourceNotFound(String),
    UserNotFound(String),
    GroupNotFound(String),
    MissingPolicy(String),
    EffectNotFound(String),
    ManagerApi(String),
    Deserialize(String),
    OtherInternal(String),
    FatalError(String),
    AssertFail(String),
}

impl ProxyError {
    pub const fn name(&self) -> &str {
        match self {
            Self::BadRequest(_) => "BadRequest",
            Self::MalformedProtocol(_) => "MalformedProtocol",
            Self::InvalidEndpoint(_) => "InvalidEndpoint",
            Self::InvalidRegion(_) => "InvalidRegion",
            Self::InvalidAuthorizationHeader(_) => "InvalidAuthorizationHeader",
            Self::InvalidAccessKey(_) => "InvalidAccessKey",
            Self::ParserError(_) => "ParserError",
            Self::OperationNotSupported(_) => "OperationNotSupported",
            Self::ResourceNotFound(_) => "ResourceNotFound",
            Self::UserNotFound(_) => "UserNotFound",
            Self::GroupNotFound(_) => "GroupNotFound",
            Self::MissingPolicy(_) => "MissingPolicy",
            Self::EffectNotFound(_) => "EffectNotFound",
            Self::ManagerApi(_) => "ManagerApi",
            Self::Deserialize(_) => "Deserialize",
            Self::OtherInternal(_) => "OtherInternal",
            Self::FatalError(_) => "FatalError",
            Self::AssertFail(_) => "AssertFail",
        }
    }
}

impl Display for ProxyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadRequest(msg) => write!(f, "BadRequest: {msg}"),
            Self::MalformedProtocol(msg) => write!(f, "MalformedProtocol: {msg}"),
            Self::InvalidEndpoint(_) => write!(f, "InvalidEndpoint"),
            Self::InvalidRegion(msg) => write!(f, "InvalidRegion: {msg}"),
            Self::InvalidAuthorizationHeader(msg) => {
                write!(f, "InvalidAuthorizationHeader: {msg}")
            }
            Self::InvalidAccessKey(msg) => write!(f, "InvalidAccessKey: {msg}"),
            Self::ParserError(msg) => write!(f, "ParserError: {msg}"),
            Self::OperationNotSupported(msg) => write!(f, "OperationNotSupported: {msg}"),
            Self::ResourceNotFound(msg) => write!(f, "ResourceNotFound: {msg}"),
            Self::UserNotFound(msg) => write!(f, "UserNotFound: {msg}"),
            Self::GroupNotFound(msg) => write!(f, "GroupNotFound: {msg}"),
            Self::MissingPolicy(msg) => write!(f, "MissingPolicy: {msg}"),
            Self::EffectNotFound(msg) => write!(f, "EffectNotFound: {msg}"),
            Self::ManagerApi(msg) => write!(f, "ManagerApi: {msg}"),
            Self::Deserialize(msg) => write!(f, "Deserialize: {msg}"),
            Self::OtherInternal(msg) => write!(f, "OtherInternal: {msg}"),
            Self::FatalError(msg) => write!(f, "FatalError: {msg}"),
            Self::AssertFail(msg) => write!(f, "AssertFail: {msg}"),
        }
    }
}

impl From<reqwest::Error> for ProxyError {
    fn from(err: reqwest::Error) -> Self {
        Self::ManagerApi(format!("reqwest error: {err}"))
    }
}

pub fn deserialize(from: &str, payload: String, err: serde_yaml::Error) -> ProxyError {
    let string = format!("from: {from}, payload: '{payload}', msg from serde_yaml: {err}");
    ProxyError::Deserialize(string)
}

pub fn assert(msg: &str) -> ProxyError {
    ProxyError::AssertFail(msg.into())
}
