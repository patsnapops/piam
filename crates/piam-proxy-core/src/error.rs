pub type ProxyResult<T> = Result<T, ProxyError>;

#[derive(Debug)]
pub enum ProxyError {
    BadRequest(String),
    InvalidAuthorizationHeader(String),
}
