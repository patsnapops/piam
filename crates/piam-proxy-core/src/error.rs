use std::fmt;

pub type ProxyResult<T> = Result<T, ProxyError>;

#[derive(Debug)]
pub enum ProxyError {
    BadRequest(String),
    OperationNotSupported(String),
    InvalidRegion(String),
    InvalidAuthorizationHeader(String),
    InvalidAccessKey(String),
    UserNotFound(String),
    GroupNotFound(String),
    MissingPolicy(String),
    EffectNotFound(String),
    ManagerApi(String),
    Deserialize(String),
    OtherInternal(String),
}

// impl Display trait for ProxyError
impl fmt::Display for ProxyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProxyError::BadRequest(msg) => write!(f, "BadRequest: {}", msg),
            ProxyError::OperationNotSupported(msg) => write!(f, "OperationNotSupported: {}", msg),
            ProxyError::InvalidRegion(msg) => write!(f, "InvalidRegion: {}", msg),
            ProxyError::InvalidAuthorizationHeader(msg) => {
                write!(f, "InvalidAuthorizationHeader: {}", msg)
            }
            ProxyError::InvalidAccessKey(msg) => write!(f, "InvalidAccessKey: {}", msg),
            ProxyError::UserNotFound(msg) => write!(f, "UserNotFound: {}", msg),
            ProxyError::GroupNotFound(msg) => write!(f, "GroupNotFound: {}", msg),
            ProxyError::MissingPolicy(msg) => write!(f, "MissingPolicy: {}", msg),
            ProxyError::EffectNotFound(msg) => write!(f, "EffectNotFound: {}", msg),
            ProxyError::ManagerApi(msg) => write!(f, "ManagerApi: {}", msg),
            ProxyError::Deserialize(msg) => write!(f, "Deserialize: {}", msg),
            ProxyError::OtherInternal(msg) => write!(f, "OtherInternal: {}", msg),
        }
    }
}

impl From<reqwest::Error> for ProxyError {
    fn from(err: reqwest::Error) -> Self {
        ProxyError::ManagerApi(format!("reqwest error: {}", err))
    }
}

pub fn deserialize(payload: String, err: serde_yaml::Error) -> ProxyError {
    let string = format!("payload: '{}', msg from serde_yaml: {}", payload, err);
    ProxyError::Deserialize(string)
}
