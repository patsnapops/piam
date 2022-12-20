use piam_common::manager_api::VERSION;
use redis::Commands;

use crate::{
    config::REDIS_ADDRESS,
    error::{ManagerError, ManagerResult},
};

pub async fn get_resource_string(key: &str) -> ManagerResult<String> {
    let client = redis::Client::open(REDIS_ADDRESS.load().as_str())
        .map_err(|e| ManagerError::Internal(format!("failed to create redis client: {}", e)))?;
    let mut con = client
        .get_connection()
        .map_err(|e| ManagerError::Internal(format!("failed to get redis connection: {}", e)))?;
    let key = format!("piam:{}:{}", VERSION, key);
    let string = con.get(&key).map_err(|e| {
        ManagerError::Internal(format!("failed to get redis key: {} error: {}", key, e))
    })?;
    Ok(string)
}
