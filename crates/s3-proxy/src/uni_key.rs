//! Special requirement for s3 proxy: Using only one access key (without account code at the end) to
//! access buckets across multiple accounts & regions for each user

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    time::Duration,
};

use aws_sdk_s3::{config::timeout::TimeoutConfig, Client, Config, Endpoint};
use aws_smithy_async::rt::sleep::TokioSleep;
use aws_types::{region::Region, Credentials};
use busylib::{
    config::dev_mode,
    http::default_reqwest_client,
    prelude::{EnhancedExpect, EnhancedUnwrap},
};
use log::debug;
use patsnap_constants::{
    region::{AP_SHANGHAI, CN_NORTHWEST_1, NA_ASHBURN, US_EAST_1},
    IP_PROVIDER,
};
use piam_core::account::aws::AwsAccount;
use piam_object_storage::input::{ActionKind, ObjectStorageInput};
use piam_proxy::{
    error::{ProxyError, ProxyResult},
    request::from_region_to_endpoint,
};
use serde::{Deserialize, Serialize};

use crate::config::CONFIG_FETCHING_TIMEOUT;

type BucketToAccessInfo = HashMap<String, Vec<AccessInfo>>;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UniKeyInfo {
    /// bucket_name to account code
    inner: BucketToAccessInfo,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AccessInfo {
    pub account: AwsAccount,
    pub region: String,
    pub endpoint: Option<String>,
}

impl UniKeyInfo {
    /// Find the account and region corresponding to the bucket,
    /// if there are multiple buckets having a same name and region parameter is specified,
    /// get the account by the specified region.
    pub fn find_access_info(
        &self,
        input: &ObjectStorageInput,
        region: &str,
    ) -> ProxyResult<&AccessInfo> {
        if input.action_kind() == ActionKind::ListBuckets {
            return Err(ProxyError::OperationNotSupported(
                "ListBuckets not supported due to uni-key feature".into(),
            ));
        }
        let bucket = input.bucket();
        let access_info_vec = self.inner.get(bucket).ok_or_else(|| {
            ProxyError::ResourceNotFound(format!("access info not found for bucket: {bucket}"))
        })?;
        if access_info_vec.len() == 1 {
            return Ok(access_info_vec.first().unwp());
        } else {
            Ok(access_info_vec
                .iter()
                .find(|access_info| access_info.region == region)
                .ok_or_else(|| {
                    ProxyError::ResourceNotFound(format!(
                        "there are more than one buckets with the same name in multiple regions, \
                        access info not found for bucket: {bucket} in region: {region}"
                    ))
                })?)
        }
    }

    pub async fn new_from(accounts: &[AwsAccount]) -> ProxyResult<Self> {
        let access_info_vec = Self::build_access_info_vec(accounts);

        let timeout_seconds = Duration::from_secs(CONFIG_FETCHING_TIMEOUT);

        let access_info_client_vec =
            Self::build_access_info_client(access_info_vec, timeout_seconds);

        let mut inner = BucketToAccessInfo::new();
        let ip_info = Self::get_ip_info().await?;

        let mut buckets_na_ashburn: HashSet<String> = HashSet::new();
        for (access_info, client) in access_info_client_vec? {
            let mut buckets = Self::get_buckets(&access_info, &client, &ip_info).await?;

            // ? This is a workaround due to an unverified inconsistent behavior of the Tencent COS API.
            // Drop non-cn buckets for tencent buckets in cn region.
            // TODO: try remove this
            if access_info.region == NA_ASHBURN {
                buckets_na_ashburn = HashSet::from_iter(buckets.iter().cloned());
            }
            if access_info.region == AP_SHANGHAI {
                let mut buckets_ap_shanghai: HashSet<String> =
                    HashSet::from_iter(buckets.iter().cloned());
                buckets_ap_shanghai.retain(|bucket| !buckets_na_ashburn.contains(bucket));
                buckets = Vec::from_iter(buckets_ap_shanghai.iter().cloned());
            }

            buckets.into_iter().for_each(|bucket| {
                let access_info = access_info.clone();
                match inner.get_mut(&bucket) {
                    None => {
                        inner.insert(bucket, vec![access_info]);
                    }
                    Some(access_info_vec) => access_info_vec.push(access_info),
                };
            });
        }

        Ok(Self { inner })
    }

    fn build_access_info_vec(accounts: &[AwsAccount]) -> ProxyResult<Vec<AccessInfo>> {
        let access_info_vec: ProxyResult<Vec<AccessInfo>> = accounts
            .iter()
            .map(|account| {
                let account = account.clone();
                // TODO: refactor this quick and dirty solution for s3 uni-key feature
                match &account.id {
                    id if id.starts_with("cn_aws") => Ok(AccessInfo {
                        account,
                        region: CN_NORTHWEST_1.to_string(),
                        endpoint: None,
                    }),
                    id if id.starts_with("us_aws") => Ok(AccessInfo {
                        account,
                        region: US_EAST_1.to_string(),
                        endpoint: None,
                    }),
                    id if id.starts_with("cn_tencent") => Ok(AccessInfo {
                        account,
                        region: AP_SHANGHAI.to_string(),
                        endpoint: Some(from_region_to_endpoint(AP_SHANGHAI)?),
                    }),
                    id if id.starts_with("us_tencent") => Ok(AccessInfo {
                        account,
                        region: NA_ASHBURN.to_string(),
                        endpoint: Some(from_region_to_endpoint(NA_ASHBURN)?),
                    }),
                    _ => Err(ProxyError::AssertFail(format!(
                        "match region failed, unsupported account id: {}",
                        &account.code
                    )))?,
                }
            })
            .collect();
        access_info_vec
    }

    fn build_access_info_client(
        access_info_vec: ProxyResult<Vec<AccessInfo>>,
        timeout_seconds: Duration,
    ) -> ProxyResult<Vec<(AccessInfo, Client)>> {
        let access_info_client_vec: ProxyResult<Vec<(AccessInfo, Client)>> = access_info_vec?
            .into_iter()
            .map(|access| {
                let creds = Credentials::from_keys(
                    &access.account.access_key,
                    &access.account.secret_key,
                    None,
                );
                let cb = Config::builder()
                    .credentials_provider(creds)
                    .region(Region::new(access.region.clone()));
                let config = match &access.endpoint {
                    // TODO: refactor this quick and dirty solution for s3 uni-key feature
                    None => cb.build(),
                    Some(tencent_ep) => cb
                        .sleep_impl(Arc::new(TokioSleep::default()))
                        .timeout_config(
                            TimeoutConfig::builder()
                                .operation_timeout(timeout_seconds)
                                .build(),
                        )
                        .endpoint_resolver(Endpoint::immutable(tencent_ep).unwp())
                        .build(),
                };
                Ok((access, Client::from_conf(config)))
            })
            .collect();
        access_info_client_vec
    }

    async fn get_ip_info() -> ProxyResult<String> {
        if !dev_mode() {
            debug!("start fetching ip info");
        }
        let ip_info = default_reqwest_client()
            .get(IP_PROVIDER)
            .header("User-Agent", "curl")
            .send()
            .await?
            .text()
            .await?
            // 20221222: remove special characters in response of cip.cc (IP_PROVIDER)
            .replace(['\n', '\t'], "");
        if !dev_mode() {
            debug!("end fetching ip info");
        }
        Ok(ip_info)
    }

    async fn get_buckets(
        access_info: &AccessInfo,
        client: &Client,
        ip_info: &str,
    ) -> ProxyResult<Vec<String>> {
        if !dev_mode() {
            debug!(
                "start fetching uni-key info of account: {} region: {}",
                access_info.account, access_info.region
            );
        }
        let buckets = client
            .list_buckets()
            .send()
            .await
            .map_err(|e| {
                ProxyError::OtherInternal(format!(
                    "failed to get buckets for account: {} access_key: {} region: {} Error: {}, \
                         normally it is caused by permissions not configured for the account, \
                         try check the IP whitelist on peer, ip_info: {}",
                    access_info.account.code,
                    access_info.account.access_key,
                    access_info.region,
                    e,
                    ip_info
                ))
            })?
            .buckets
            .ok_or_else(|| ProxyError::AssertFail("no buckets found".into()))?
            .into_iter()
            .map(|b| b.name.ex("bucket must have a name"))
            .collect();
        if !dev_mode() {
            debug!(
                "end fetching uni-key info of account: {} region: {}",
                access_info.account, access_info.region
            );
        }
        Ok(buckets)
    }
}
