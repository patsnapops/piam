#[derive(Debug)]
pub enum InputOperationError {
    InvalidBucketOp(String),
    InvalidObjectOp(String),
}