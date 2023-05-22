use std::fmt;
use busylib::http::ReqwestError;

pub type ManagerResult<T> = Result<T, ManagerError>;

#[allow(dead_code)]
pub enum ManagerError {
    BadRequest(String),
    Internal(String),
}

impl fmt::Display for ManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ManagerError::BadRequest(msg) => write!(f, "BadRequest: {msg}"),
            ManagerError::Internal(msg) => write!(f, "Internal: {msg}"),
        }
    }
}

impl From<ReqwestError> for ManagerError {
    fn from(err: ReqwestError) -> Self {
        Self::Internal(format!("reqwest error: {err}"))
    }
}
