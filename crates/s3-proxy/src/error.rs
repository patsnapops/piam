#![allow(unused)]

use piam_object_storage::policy::ObjectStoragePolicy;
use piam_core::policy::Policy;

pub type S3ProxyResult<T> = Result<T, S3ProxyError>;

#[derive(Debug)]
pub enum S3ProxyError {}
