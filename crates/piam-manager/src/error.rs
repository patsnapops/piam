pub type ManagerResult<T> = Result<T, ManagerError>;

#[allow(dead_code)]
pub enum ManagerError {
    BadRequest(String),
    Internal(String),
}
