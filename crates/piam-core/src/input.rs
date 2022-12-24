//! A parser module can be implemented to parse the request and return a [`Input`] struct.

use std::{fmt, fmt::Debug};

pub type ParserResult<T> = Result<T, ParserError>;

pub enum ParserError {
    OperationNotSupported(String),
    InvalidEndpoint(String),
    Internal(String),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::OperationNotSupported(msg) => write!(f, "OperationNotSupported: {}", msg),
            ParserError::InvalidEndpoint(msg) => write!(f, "InvalidEndpoint: {}", msg),
            ParserError::Internal(msg) => write!(f, "Internal: {}", msg),
        }
    }
}

pub trait Input: Sized + Debug {}
