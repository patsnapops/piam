use std::{fmt, fmt::Display};

use log::error;

use crate::input::ParserError;

pub type ProxyResult<T> = Result<T, ProxyError>;

#[derive(Debug)]
pub enum ProxyError {
    BadRequest(String),
    InvalidEndpoint(String),
    InvalidRegion(String),
    InvalidAuthorizationHeader(String),
    InvalidAccessKey(String),
    ParserError(String),
    OperationNotSupported(String),
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
    pub fn name(&self) -> &str {
        match self {
            ProxyError::BadRequest(_) => "BadRequest",
            ProxyError::InvalidEndpoint(_) => "InvalidEndpoint",
            ProxyError::InvalidRegion(_) => "InvalidRegion",
            ProxyError::InvalidAuthorizationHeader(_) => "InvalidAuthorizationHeader",
            ProxyError::InvalidAccessKey(_) => "InvalidAccessKey",
            ProxyError::ParserError(_) => "ParserError",
            ProxyError::OperationNotSupported(_) => "OperationNotSupported",
            ProxyError::UserNotFound(_) => "UserNotFound",
            ProxyError::GroupNotFound(_) => "GroupNotFound",
            ProxyError::MissingPolicy(_) => "MissingPolicy",
            ProxyError::EffectNotFound(_) => "EffectNotFound",
            ProxyError::ManagerApi(_) => "ManagerApi",
            ProxyError::Deserialize(_) => "Deserialize",
            ProxyError::OtherInternal(_) => "OtherInternal",
            ProxyError::FatalError(_) => "FatalError",
            ProxyError::AssertFail(_) => "AssertFail",
        }
    }
}

// impl Display trait for ProxyError
impl Display for ProxyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProxyError::BadRequest(msg) => write!(f, "BadRequest: {}", msg),
            ProxyError::InvalidEndpoint(_) => write!(f, "InvalidEndpoint"),
            ProxyError::InvalidRegion(msg) => write!(f, "InvalidRegion: {}", msg),
            ProxyError::InvalidAuthorizationHeader(msg) => {
                write!(f, "InvalidAuthorizationHeader: {}", msg)
            }
            ProxyError::InvalidAccessKey(msg) => write!(f, "InvalidAccessKey: {}", msg),
            ProxyError::ParserError(msg) => write!(f, "ParserError: {}", msg),
            ProxyError::OperationNotSupported(msg) => write!(f, "OperationNotSupported: {}", msg),
            ProxyError::UserNotFound(msg) => write!(f, "UserNotFound: {}", msg),
            ProxyError::GroupNotFound(msg) => write!(f, "GroupNotFound: {}", msg),
            ProxyError::MissingPolicy(msg) => write!(f, "MissingPolicy: {}", msg),
            ProxyError::EffectNotFound(msg) => write!(f, "EffectNotFound: {}", msg),
            ProxyError::ManagerApi(msg) => write!(f, "ManagerApi: {}", msg),
            ProxyError::Deserialize(msg) => write!(f, "Deserialize: {}", msg),
            ProxyError::OtherInternal(msg) => write!(f, "OtherInternal: {}", msg),
            ProxyError::FatalError(msg) => write!(f, "FatalError: {}", msg),
            ProxyError::AssertFail(msg) => write!(f, "AssertFail: {}", msg),
        }
    }
}

impl From<ParserError> for ProxyError {
    fn from(err: ParserError) -> Self {
        match err {
            ParserError::OperationNotSupported(msg) => ProxyError::OperationNotSupported(msg),
            ParserError::InvalidEndpoint(msg) => ProxyError::InvalidEndpoint(msg),
            ParserError::Internal(msg) => ProxyError::ParserError(msg),
        }
    }
}

impl From<reqwest::Error> for ProxyError {
    fn from(err: reqwest::Error) -> Self {
        ProxyError::ManagerApi(format!("reqwest error: {}", err))
    }
}

pub fn deserialize(from: &str, payload: String, err: serde_yaml::Error) -> ProxyError {
    let string = format!(
        "from: {}, payload: '{}', msg from serde_yaml: {}",
        from, payload, err
    );
    ProxyError::Deserialize(string)
}

pub fn assert(msg: &str) -> ProxyError {
    ProxyError::AssertFail(msg.into())
}

/// eok stands for it is expected to be [`Ok`], unexpected [`Err`] will be logged.
pub fn eok<T, E: Display>(result: Result<T, E>) -> T {
    match result {
        Ok(value) => value,
        Err(e) => {
            error!("this should not happen: {}", e);
            panic!("this should not happen: {}", e);
        }
    }
}

/// esome stands for it is expected to be [`Some`], unexpected [`None`] will be logged.
pub fn esome<T>(option: Option<T>) -> T {
    match option {
        Some(value) => value,
        None => {
            error!("this should not happen: None");
            panic!("this should not happen: None");
        }
    }
}

pub fn eok_ctx<T, E: Display>(result: Result<T, E>, msg: &str) -> T {
    match result {
        Ok(value) => value,
        Err(e) => {
            error!("this should not happen: {}, context: {}", e, msg);
            panic!("this should not happen: {}, context: {}", e, msg);
        }
    }
}

pub fn esome_ctx<T>(option: Option<T>, msg: &str) -> T {
    match option {
        Some(value) => value,
        None => {
            error!("this should not happen: None, context: {}", msg);
            panic!("this should not happen: None, context: {}", msg);
        }
    }
}
