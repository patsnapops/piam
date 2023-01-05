use async_trait::async_trait;
use busylib::config::dev_mode;
use log::debug;
use piam_object_storage::{config::HostDomains, policy::ObjectStoragePolicy};
use piam_proxy::{
    config::CoreConfig,
    error::{ProxyError, ProxyResult},
    state::ExtendedState,
};
use serde::{Deserialize, Serialize};

pub const CONFIG_FETCHING_TIMEOUT: u64 = 10;
pub const DEV_PROXY_HOST: &str = "s3-proxy.dev";
pub const SERVICE: &str = "s3";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct S3Config {
    pub proxy_hosts: HostDomains,
    #[cfg(feature = "uni-key")]
    pub uni_key_info: Option<crate::uni_key::UniKeyInfo>,
}

#[async_trait]
impl ExtendedState<S3Config, ObjectStoragePolicy> for S3Config {
    fn new_from(mut extended_config: S3Config) -> ProxyResult<Self> {
        if dev_mode() {
            extended_config
                .proxy_hosts
                .domains
                .push(DEV_PROXY_HOST.to_string());
        }
        Ok(extended_config)
    }

    async fn with_core_config(
        mut self,
        core_config: &CoreConfig<ObjectStoragePolicy>,
    ) -> ProxyResult<Self> {
        #[cfg(feature = "uni-key")]
        {
            if !dev_mode() {
                debug!("start fetching uni-key info");
            }
            self.uni_key_info =
                Some(crate::uni_key::UniKeyInfo::new_from(&core_config.accounts).await?);
            if !dev_mode() {
                debug!("end fetching uni-key info");
            }
            return Ok(self);
        };
        #[cfg(not(feature = "uni-key"))]
        Ok(self)
    }
}

impl S3Config {
    #[cfg(feature = "uni-key")]
    pub fn get_uni_key_info(&self) -> ProxyResult<&crate::uni_key::UniKeyInfo> {
        self.uni_key_info
            .as_ref()
            .ok_or_else(|| ProxyError::AssertFail("UniKeyInfo not found".into()))
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

#[cfg(test)]
mod test {
    use piam_object_storage::config::HostDomains;

    #[test]
    fn find_proxy_host() {
        let config = crate::config::S3Config {
            proxy_hosts: HostDomains {
                domains: vec!["cn-northwest-1.s3-proxy.patsnap.info".into()],
            },
            uni_key_info: None,
        };
        let result = config.proxy_hosts.find_proxy_host(
            "datalake-internal.patsnap.com-cn-northwest-1.cn-northwest-1.s3-proxy.patsnap.info",
        );
        assert!(result.is_ok())
    }
}
