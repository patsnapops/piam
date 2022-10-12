pub type ProxyResult<T> = Result<T, ProxyError>;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum ProxyError {
    InvalidRequest(String),
}
