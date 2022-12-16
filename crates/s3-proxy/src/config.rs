use async_trait::async_trait;
use piam_proxy_core::{
    config::{dev_mode, ParserConfig},
    error::{ProxyError, ProxyResult},
    manager_api::ManagerClient,
    state::GetNewState,
};
use serde::{Deserialize, Serialize};

pub const DEV_PROXY_HOST: &str = "s3-proxy.dev";
pub const SERVICE: &str = "s3";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct S3Config {
    pub proxy_hosts: Vec<String>,
    #[cfg(feature = "uni-key")]
    pub uni_key_info: Option<crate::uni_key::UniKeyInfo>,
}

impl ParserConfig for S3Config {}

#[async_trait]
impl GetNewState for S3Config {
    async fn new_from_manager(manager: &ManagerClient) -> ProxyResult<Self> {
        let mut config: S3Config = manager.get_extended_config(SERVICE).await?;
        if dev_mode() {
            config.proxy_hosts.push(DEV_PROXY_HOST.to_string());
        }
        #[cfg(feature = "uni-key")]
        let config = {
            config.uni_key_info = Some(crate::uni_key::UniKeyInfo::new_from(manager).await?);
            config
        };
        Ok(config)
    }
}

impl S3Config {
    pub fn find_proxy_host(&self, host: &str) -> ProxyResult<&str> {
        let s = self
            .proxy_hosts
            .iter()
            .find(|&v| host.ends_with(v))
            .ok_or_else(|| {
                ProxyError::InvalidEndpoint(format!(
                    "'{}' is not ending with a valid piam s3 proxy endpoint",
                    host
                ))
            })?;
        Ok(s)
    }

    #[cfg(feature = "uni-key")]
    pub fn get_uni_key_info(&self) -> ProxyResult<&crate::uni_key::UniKeyInfo> {
        self.uni_key_info
            .as_ref()
            .ok_or_else(|| ProxyError::OtherInternal("UniKeyInfo not found".into()))
    }
}

pub fn features() -> String {
    let features = vec![
        #[cfg(feature = "uni-key")]
        "uni-key",
    ];
    let mut list = "[".to_string();
    for feature in features {
        list.push_str(feature);
        list.push_str(", ");
    }
    list.pop();
    list.pop();
    list.push(']');
    list
}

pub mod test {
    #[test]
    fn find_proxy_host() {
        let config = crate::config::S3Config {
            proxy_hosts: vec!["cn-northwest-1.s3-proxy.patsnap.info".into()],
            uni_key_info: None,
        };
        let result = config.find_proxy_host(
            "datalake-internal.patsnap.com-cn-northwest-1.cn-northwest-1.s3-proxy.patsnap.info",
        );
        assert!(result.is_ok())
    }
}