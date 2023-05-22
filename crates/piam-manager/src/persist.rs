use piam_core::manager_api_constant::VERSION;
use redis::{Client, Commands};
use busylib::http::{
    default_reqwest_client,
    ReqwestClient,
};
use log::{info, warn};
use std::option::Option;

use crate::{
    config::{REDIS_ADDRESS, PORTAL_ADDRESS, PORTAL_API_VERSION, PORTAL_API_PATH},
    error::{ManagerError, ManagerResult},
};

pub async fn get_resource_string(key: &str) -> ManagerResult<String> {
    let string = ManagerResourceClient::new().get_resource_string(key).await.map_err(|e| {
        warn!("failed to get resource configurations, error: {}", e);
        e
    })?;
    Ok(string)
}

pub struct ManagerResourceClient {
    http_client: Option<ReqwestClient>,
    redis_client: Option<Client>,
}

impl ManagerResourceClient {
    pub fn new() -> Self {
        Self {
            http_client: Some(default_reqwest_client()),
            redis_client: Client::open(REDIS_ADDRESS.load().as_str()).ok(),
        }
    }

    pub async fn get_resource_string(&self, key: &str) -> Result<String, ManagerError> {
        match self.get_resource_from_portal(key).await {
            Ok(result) => Ok(result),
            Err(err) => {
                warn!("failed to get resource from portal: {}", err);
                info!("try to get resource from redis");
                match self.get_resource_from_redis(key).await {
                    Ok(result) => Ok(result),
                    Err(err) => {
                        warn!("failed to get resource from redis: {}", err);
                        Err(err)
                    }
                }
            }
        }
    }

    async fn get_resource_from_redis(&self, key: &str) -> Result<String, ManagerError> {
        let client = self.redis_client.as_ref().ok_or_else(|| {
            ManagerError::Internal("redis client is not initialized".to_owned())
        })?;

        let key = format!("piam:{}:{}", VERSION, key);
        let mut con = client
            .get_connection()
            .map_err(|e| ManagerError::Internal(format!("failed to get redis connection: {}", e)))?;
        let string: Option<String> = con.get(&key).map_err(|e| {
            ManagerError::Internal(format!("failed to get redis key: {} error: {}", key, e))
        })?;
        string.ok_or_else(|| ManagerError::BadRequest(key.to_owned()))
    }

    async fn get_resource_from_portal(&self, key: &str) -> Result<String, ManagerError> {
        let path = PORTAL_API_PATH.get(key).ok_or_else(|| {
            ManagerError::BadRequest(format!("failed to get resource from portal, key: {}", key))
        })?;

        let url = format!("{}/{}/piam/{}", PORTAL_ADDRESS.load(), PORTAL_API_VERSION.load(), path);
        let client = self.http_client.as_ref().ok_or_else(|| {
            ManagerError::Internal("http client is not initialized".to_owned())
        })?;
        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| ManagerError::Internal(format!("failed to get portal resource: {}", e)))?
            .error_for_status()
            .map_err(|e| ManagerError::Internal(format!("failed to get portal resource: {}", e)))?
            .text()
            .await
            .map_err(|e| ManagerError::Internal(format!("failed to get portal resource: {}", e)))?;

        Ok(response)
    }
}
