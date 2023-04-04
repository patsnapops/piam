pub type PiamResult<T> = Result<T, PiamError>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PiamError {
    Conflict(String),
}

impl std::fmt::Display for PiamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Conflict(msg) => write!(f, "Conflict: {}", msg),
        }
    }
}
