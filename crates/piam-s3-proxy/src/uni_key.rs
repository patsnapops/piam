//! Special requirement for s3 proxy: Using only one access key (without account code at the end) to
//! access buckets across multiple accounts & regions for each user

use std::collections::HashMap;

use aws_sdk_s3::{Client, Config, Endpoint};
use aws_types::{region::Region, Credentials};
use log::debug;
use piam_proxy_core::{
    account::{aws::AwsAccount, AccountId},
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

type BucketToAccount = HashMap<String, AwsAccount>;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UniKeyInfo {
    /// bucket_name to account code
    inner: BucketToAccount,
}

#[derive(Debug, Default)]
struct SdkClientConf {
    account: AwsAccount,
    region: String,
    endpoint: Option<String>,
}

impl UniKeyInfo {
    pub fn find_account_by_input(&self, s3_input: &S3Input) -> ProxyResult<&AwsAccount> {
        if s3_input.action_kind() == ActionKind::ListBuckets {
            return Err(ProxyError::OperationNotSupported(
                "ListBuckets not supported due to uni-key feature".into(),
            ));
        }
        let bucket = s3_input.bucket();
        let account = self.inner.get(bucket).ok_or_else(|| {
            ProxyError::BadRequest(format!("account not found for bucket: {}", bucket))
        })?;
        Ok(account)
    }

    pub async fn new_from(manager: &ManagerClient) -> ProxyResult<Self> {
        let accounts = manager.get_accounts().await?;
        let vec_conf: ProxyResult<Vec<SdkClientConf>> = accounts
            .into_iter()
            .map(|account| {
                match &account.id {
                    id if id.starts_with("cn_aws") => Ok(SdkClientConf {
                        account,
                        region: "cn-northwest-1".to_string(),
                        endpoint: None,
                    }),
                    id if id.starts_with("us_aws") => Ok(SdkClientConf {
                        account,
                        region: "us-east-1".to_string(),
                        endpoint: None,
                    }),
                    // TODO: refactor this quick and dirty solution for s3 uni-key feature
                    id if id.starts_with("cn_tencent") => Ok(SdkClientConf {
                        account,
                        region: "ap-shanghai".to_string(),
                        endpoint: Some(from_region_to_endpoint("ap-shanghai")?),
                    }),
                    id if id.starts_with("us_tencent") => Ok(SdkClientConf {
                        account,
                        region: "na-ashburn".to_string(),
                        endpoint: Some(from_region_to_endpoint("na-ashburn")?),
                    }),
                    _ => Err(ProxyError::OtherInternal(format!(
                        "match region failed, unsupported account id: {}",
                        &account.code
                    )))?,
                }
            })
            .collect();

        let vec_account_region_client: ProxyResult<Vec<(SdkClientConf, Client)>> = vec_conf?
            .into_iter()
            .map(|conf| {
                let creds =
                    Credentials::from_keys(&conf.account.ak_id, &conf.account.secret_key, None);
                let cb = Config::builder()
                    .credentials_provider(creds)
                    .region(Region::new(conf.region.clone()));
                let config = match &conf.endpoint {
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
                Ok((conf, Client::from_conf(config)))
            })
            .collect();

        let mut inner = BucketToAccount::new();
        for (client_conf, client) in vec_account_region_client? {
            let buckets = Self::get_buckets(&client_conf, &client)
                .await
                .map_err(|e| {
                    ProxyError::OtherInternal(format!(
                        "failed to get buckets for account: {} ak_id: {} region: {} Error: {}",
                        client_conf.account.code, client_conf.account.ak_id, client_conf.region, e
                    ))
                })?;
            buckets.into_iter().for_each(|bucket| {
                inner.insert(bucket, client_conf.account.clone());
            });
        }

        Ok(Self { inner })
    }

    async fn get_buckets(
        client_conf: &SdkClientConf,
        client: &Client,
    ) -> ProxyResult<Vec<String>> {
        dbg!(&client_conf);
        let buckets = client
            .list_buckets()
            .send()
            .await
            .map_err(|e| {
                ProxyError::OtherInternal(format!(
                    "client_conf: {:#?} failed to list buckets: {}",
                    client_conf, e
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
