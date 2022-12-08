//! Special requirement for s3 proxy: Using only one access key (without account code at the end) to
//! access buckets across multiple accounts & regions for each user

use std::collections::HashMap;

use aws_sdk_s3::{Client, Config, Endpoint};
use aws_types::{region::Region, Credentials};
use log::debug;
use piam_proxy_core::{
    account::{aws::AwsAccount, AccountId},
    config::{AP_SHANGHAI, CN_NORTHWEST_1, NA_ASHBURN, US_EAST_1},
    container::IamContainer,
    error::{ProxyError, ProxyResult},
    manager_api::ManagerClient,
    request::{from_region_to_endpoint, from_region_to_host},
};
use serde::{Deserialize, Serialize};

use crate::{
    parser::{ActionKind, S3Input},
    policy::S3Statement,
    S3Config,
};

type BucketToAccessInfo = HashMap<String, AccessInfo>;

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
    pub fn find_access_info_input(&self, s3_input: &S3Input) -> ProxyResult<&AccessInfo> {
        if s3_input.action_kind() == ActionKind::ListBuckets {
            return Err(ProxyError::OperationNotSupported(
                "ListBuckets not supported due to uni-key feature".into(),
            ));
        }
        let bucket = s3_input.bucket();
        let access_info = self.inner.get(bucket).ok_or_else(|| {
            ProxyError::BadRequest(format!("access info not found for bucket: {}", bucket))
        })?;
        Ok(access_info)
    }

    pub async fn new_from(manager: &ManagerClient) -> ProxyResult<Self> {
        let accounts = manager.get_accounts().await?;
        let access_info_vec: ProxyResult<Vec<AccessInfo>> = accounts
            .into_iter()
            .map(|account| {
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
                    // TODO: refactor this quick and dirty solution for s3 uni-key feature
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
                    _ => Err(ProxyError::OtherInternal(format!(
                        "match region failed, unsupported account id: {}",
                        &account.code
                    )))?,
                }
            })
            .collect();

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
                    Some(tep) => {
                        let uri = tep.parse().map_err(|e| {
                            ProxyError::OtherInternal(format!(
                                "invalid URI for tencent endpoint: {}",
                                e
                            ))
                        })?;
                        cb.endpoint_resolver(Endpoint::immutable(uri)).build()
                    }
                };
                Ok((access, Client::from_conf(config)))
            })
            .collect();

        let mut inner = BucketToAccessInfo::new();
        for (access_info, client) in access_info_client_vec? {
            let buckets = Self::get_buckets(&access_info, &client)
                .await
                .map_err(|e| {
                    ProxyError::OtherInternal(format!(
                        "failed to get buckets for account: {} access_key: {} region: {} Error: {}",
                        access_info.account.code,
                        access_info.account.access_key,
                        access_info.region,
                        e
                    ))
                })?;
            buckets.into_iter().for_each(|bucket| {
                inner.insert(bucket, access_info.clone());
            });
        }

        Ok(Self { inner })
    }

    async fn get_buckets(client_conf: &AccessInfo, client: &Client) -> ProxyResult<Vec<String>> {
        let buckets = client
            .list_buckets()
            .send()
            .await
            .map_err(|e| {
                ProxyError::OtherInternal(format!(
                    "account.access_key: {} failed to list buckets: {}",
                    client_conf.account.access_key, e
                ))
            })?
            .buckets
            .ok_or_else(|| ProxyError::OtherInternal("no buckets found".into()))?
            .into_iter()
            .map(|b| b.name.expect("bucket should always have name"))
            .collect();
        Ok(buckets)
    }
}
