pub type S3ProxyResult<T> = Result<T, S3ProxyError>;

#[derive(Debug)]
pub enum S3ProxyError {}
