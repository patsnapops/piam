//! Special requirement for s3 proxy: Using only one access key (without account code at the end) to
//! access buckets across multiple accounts & regions for each user

use std::collections::HashMap;

use aws_sdk_s3::{Client, Config};
use aws_types::{region::Region, Credentials};
use log::debug;
use piam_proxy_core::{
    account::{aws::AwsAccount, AccountId},
    container::IamContainer,
    error::{ProxyError, ProxyResult},
    manager_api::ManagerClient,
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

struct AwsAccountRegion {
    account: AwsAccount,
    region: String,
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
        let vec_account_region: ProxyResult<Vec<AwsAccountRegion>> = accounts
            .into_iter()
            .map(|account| {
                let region = match &account.id {
                    id if id.starts_with("cn") => "cn-north-1",
                    id if id.starts_with("us") => "us-east-1",
                    _ => Err(ProxyError::OtherInternal(format!(
                        "unsupported account id: {}",
                        &account.code
                    )))?,
                }
                .to_string();
                Ok(AwsAccountRegion { account, region })
            })
            .collect();

        let vec_account_region_client: Vec<(AwsAccountRegion, Client)> = vec_account_region?
            .into_iter()
            .map(|ar| {
                let creds = Credentials::from_keys(&ar.account.ak_id, &ar.account.secret_key, None);
                let config = Config::builder()
                    .credentials_provider(creds)
                    // TODO: see if one region is enough for ListBuckets
                    .region(Region::new(ar.region.clone()))
                    .build();
                (ar, Client::from_conf(config))
            })
            .collect();

        let mut inner = BucketToAccount::new();
        for (account_region, client) in vec_account_region_client {
            let buckets = Self::get_buckets(&client).await.map_err(|e| {
                ProxyError::OtherInternal(format!(
                    "failed to get buckets for account: {} region: {} Error: {}",
                    account_region.account.code, account_region.region, e
                ))
            })?;
            buckets.into_iter().for_each(|bucket| {
                inner.insert(bucket, account_region.account.clone());
            });
        }

        Ok(Self { inner })
    }

    async fn get_buckets(client: &Client) -> ProxyResult<Vec<String>> {
        let buckets = client
            .list_buckets()
            .send()
            .await
            .map_err(|e| ProxyError::OtherInternal(format!("failed to list buckets: {}", e)))?
            .buckets
            .ok_or_else(|| ProxyError::OtherInternal("no buckets found".into()))?
            .into_iter()
            .map(|b| b.name.expect("bucket should always have name"))
            .collect();
        Ok(buckets)
    }
}
