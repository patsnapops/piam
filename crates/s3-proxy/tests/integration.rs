#![allow(unused)]

use aws_config::{from_env, provider_config::ProviderConfig};
use aws_sdk_s3::{
    error::HeadBucketErrorKind,
    model::{CompletedMultipartUpload, CompletedPart, Object},
    types::{ByteStream, SdkError},
    Client, Config, Endpoint,
};
use aws_smithy_client::{erase::DynConnector, never::NeverConnector};
use aws_types::{os_shim_internal::Env, region::Region, Credentials};
use futures::future;
use piam_proxy_core::config::{AP_SHANGHAI, CN_NORTHWEST_1, NA_ASHBURN, US_EAST_1};
use uuid::Uuid;

pub const DEV_PROXY_HOST: &str = "s3-proxy.dev";
pub const DEV_PROXY_ENDPOINT: &str = "http://s3-proxy.dev";
pub const EP_AP_SHANGHAI: &str = "http://ap-shanghai.s3-proxy.patsnap.info";

const REAL_ACCESS_KEY_ID: &str = "";
const REAL_SECRET_ACCESS_KEY: &str = "";

// only ListBuckets does not have bucket name in url or host
#[tokio::test]
async fn list_buckets() {
    let output = build_client().await.list_buckets().send().await.unwrap();
    let buckets = output.buckets().unwrap();
    assert!(buckets.len() > 10);
}

#[tokio::test]
async fn head_bucket() {
    let client = build_client().await;
    let output = client.head_bucket().bucket("anniversary").send().await;
    assert!(output.is_ok());
    let output = client
        .head_bucket()
        .bucket(Uuid::new_v4().to_string())
        .send()
        .await;
    match output.err().unwrap() {
        SdkError::ServiceError { err, raw } => {
            assert!(matches!(err.kind, HeadBucketErrorKind::NotFound(not_found)));
        }
        _ => {
            panic!("unexpected test error");
        }
    }
}

#[tokio::test]
async fn get_bucket_tagging() {
    let output = build_client()
        .await
        .get_bucket_tagging()
        .bucket("api.patsnap.info")
        .send()
        .await
        .unwrap();
    let tag_set = output.tag_set().unwrap();
    assert!(tag_set.len() > 1);
}

#[tokio::test]
async fn get_bucket_notification_configuration() {
    let output = build_client()
        .await
        .get_bucket_notification_configuration()
        .bucket("api.patsnap.info")
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn list_objects_v1() {
    let output = build_client()
        .await
        .list_objects()
        .bucket("anniversary")
        .prefix("image")
        .send()
        .await
        .unwrap();
    assert!(output.contents().unwrap().len() > 2);
}

#[tokio::test]
async fn list_objects_v2() {
    let output = build_client()
        .await
        .list_objects_v2()
        .bucket("anniversary")
        .send()
        .await
        .unwrap();
    assert!(output.key_count() > 10);
}

#[tokio::test]
async fn get_object_inside_folder() {
    let output = build_client()
        .await
        .get_object()
        .bucket("anniversary")
        .key("__MACOSX/image/._.DS_Store")
        .part_number(1)
        .send()
        .await
        .unwrap();

    let size = output.content_length();
    assert!(size > 10);
}

#[tokio::test]
async fn get_object_with_domain_bucket_name() {
    let output = build_client()
        .await
        .get_object()
        .bucket("api.patsnap.info")
        .key("index.html")
        .send()
        .await
        .unwrap();
    let size = output.content_length();
    assert!(size > 10);
}

async fn get_object_acl() {
    let output = build_client()
        .await
        .get_object_acl()
        .bucket("api.patsnap.info")
        .key("index.html")
        .send()
        .await
        .unwrap();
    let g = output.grants().unwrap();
    dbg!(g);
}

#[tokio::test]
async fn put_object() {
    // put_object_random_key("qa-ops-test-s3").await;
    // fixme: support special char like *
    // put_object_with_key("中").await;
    // put_object_with_key("*").await;
    put_object_with_key("test1019").await;
}

#[tokio::test]
async fn copy_object() {
    let bucket = "qa-ops-test-s3";
    let key = put_object_random_key(bucket).await;

    let output = build_client()
        .await
        .copy_object()
        .bucket(bucket)
        .key(format!("patsnap-s3-proxy/{}", "dst_key_for_copy_test"))
        .copy_source(format!("{}/{}", bucket, key))
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn delete_object() {
    let bucket = "qa-ops-test-s3";
    let key = put_object_random_key(bucket).await;

    let output = build_client()
        .await
        .delete_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn create_multipart_upload() {
    let (bucket, key, upload_id) = do_create_multipart_upload().await;
    assert!(upload_id.len() > 10);
}

async fn do_create_multipart_upload() -> (&'static str, &'static str, String) {
    let bucket = "qa-ops-test-s3";
    let key = "patsnap-s3-proxy/multipart-file";
    let output = build_client()
        .await
        .create_multipart_upload()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .unwrap();
    (bucket, key, output.upload_id().unwrap().to_string())
}

async fn upload_parts() -> (&'static str, &'static str, String, Vec<CompletedPart>) {
    let (bucket, key, upload_id) = do_create_multipart_upload().await;

    async fn upload_part(
        bucket: &str,
        key: &str,
        part_number: i32,
        upload_id: &String,
    ) -> Result<CompletedPart, ()> {
        // part size must >= 5MB
        const SIZE: usize = 5 * 1024 * 1024;
        let body = ByteStream::from(vec![1; SIZE]);
        let output = build_client()
            .await
            .upload_part()
            .bucket(bucket)
            .key(key)
            .upload_id(upload_id)
            .part_number(part_number)
            .body(body)
            .send()
            .await
            .unwrap();
        let part = CompletedPart::builder()
            .part_number(part_number)
            .e_tag(output.e_tag().unwrap())
            .build();
        Ok(part)
    }

    let n = vec![1, 2];
    let map = n.iter().map(|n| upload_part(bucket, key, *n, &upload_id));
    let parts = future::try_join_all(map).await.unwrap();

    (bucket, key, upload_id, parts)
}

#[tokio::test]
async fn _slow_multipart_upload_big_file() {
    let (bucket, key, upload_id, parts) = upload_parts().await;
    let cmu = CompletedMultipartUpload::builder()
        .set_parts(Some(parts))
        .build();
    let output = build_client()
        .await
        .complete_multipart_upload()
        .bucket(bucket)
        .key(key)
        .upload_id(upload_id)
        .multipart_upload(cmu)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn _slow_list_multipart_uploads() {
    let (bucket, key, upload_id, parts) = upload_parts().await;
    let output = build_client()
        .await
        .list_multipart_uploads()
        .bucket(bucket)
        .send()
        .await
        .unwrap();
    let uploads = output.uploads().unwrap();
    assert!(!uploads.is_empty());
}

#[tokio::test]
async fn _slow_list_parts() {
    let (bucket, key, upload_id, parts) = upload_parts().await;
    let output = build_client()
        .await
        .list_parts()
        .bucket(bucket)
        .key(key)
        .upload_id(upload_id)
        .send()
        .await
        .unwrap();
    let parts = output.parts().unwrap();
    assert!(!parts.is_empty());
}

#[allow(non_snake_case)]
#[tokio::test]
async fn _slow_show_files_bigger_than_5GB() {
    let client = build_client().await;
    let output = client.list_buckets().send().await.unwrap();
    let buckets = output.buckets().unwrap();
    dbg!("buckets.len(): {:#?}", buckets.len());
    for bucket in buckets {
        let bucket_name = bucket.name().unwrap();
        let list_objects_v2output = client
            .list_objects_v2()
            .bucket(bucket_name)
            .send()
            .await
            .unwrap();
        let option = list_objects_v2output.contents();
        if let Some(objs) = option {
            let vec = objs
                .iter()
                .filter(|o| o.size() > 5_000_000_000)
                .collect::<Vec<&Object>>();

            if !vec.is_empty() {
                dbg!(bucket_name);
                for obj in vec {
                    dbg!(obj.key().unwrap());
                }
            }
        }
    }
}

async fn put_object_random_key(bucket: impl Into<std::string::String>) -> String {
    let key = format!("patsnap-s3-proxy/{}", Uuid::new_v4());
    do_put_object(bucket, key.clone()).await;
    key
}

async fn put_object_with_key(key: &str) -> String {
    do_put_object("qa-ops-test-s3", format!("patsnap-s3-proxy/{}", key)).await;
    key.to_string()
}

async fn do_put_object(bucket: impl Into<String>, key: impl Into<String>) {
    let client = build_client().await;
    let content = "dummy";
    let output = client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from_static(content.as_bytes()))
        .send()
        .await
        .unwrap();
}

async fn build_client() -> Client {
    let args: Vec<String> = std::env::args().collect();
    if let Some(real) = args.last() {
        if let "real" = real.as_str() {
            return build_real_key_to_cn_northwest_client().await;
        }
    }
    build_fake_key_to_cn_northwest_client_dev().await
}

async fn build_real_key_to_cn_northwest_client() -> Client {
    let env = Env::from_slice(&[
        ("AWS_MAX_ATTEMPTS", "1"),
        ("AWS_REGION", CN_NORTHWEST_1),
        ("AWS_ACCESS_KEY_ID", REAL_ACCESS_KEY_ID),
        ("AWS_SECRET_ACCESS_KEY", REAL_SECRET_ACCESS_KEY),
    ]);
    build_client_from_env(env, "http://s3.cn-northwest-1.amazonaws.com.cn").await
}

async fn build_fake_key_to_us_east_client_dev() -> Client {
    let env = Env::from_slice(&[
        ("AWS_MAX_ATTEMPTS", "1"),
        ("AWS_REGION", US_EAST_1),
        ("AWS_ACCESS_KEY_ID", "AKPSSVCSDATALAKE"),
        ("AWS_SECRET_ACCESS_KEY", "dummy_sk"),
    ]);
    build_client_from_env(env, &format!("http://{}", DEV_PROXY_HOST)).await
}

async fn build_fake_key_to_cn_northwest_client_dev() -> Client {
    let env = Env::from_slice(&[
        ("AWS_MAX_ATTEMPTS", "1"),
        ("AWS_REGION", CN_NORTHWEST_1),
        ("AWS_ACCESS_KEY_ID", "AKPSSVCSPROXYDEV"),
        ("AWS_SECRET_ACCESS_KEY", "dummy_sk"),
    ]);
    build_client_from_env(env, &format!("http://{}", DEV_PROXY_HOST)).await
}

async fn build_client_from_env(env: Env, endpoint: &str) -> Client {
    let conf = from_env()
        .configure(
            ProviderConfig::empty()
                .with_env(env)
                .with_http_connector(DynConnector::new(NeverConnector::new())),
        )
        .endpoint_resolver(Endpoint::immutable(endpoint.parse().expect("invalid URI")))
        .load()
        .await;
    aws_sdk_s3::Client::new(&conf)
}

pub struct ClientParams {
    pub access_key: &'static str,
    pub secret: &'static str,
    pub region: &'static str,
    pub endpoint: &'static str,
}

fn build_client_from_params(params: ClientParams) -> Client {
    let creds = Credentials::from_keys(params.access_key, params.secret, None);
    let cb = Config::builder()
        .credentials_provider(creds)
        .endpoint_resolver(Endpoint::immutable(
            params.endpoint.parse().expect("invalid URI"),
        ))
        .region(Region::new(params.region));
    Client::from_conf(cb.build())
}

async fn build_dt_us_east_client() -> Client {
    let env = Env::from_slice(&[
        ("AWS_MAX_ATTEMPTS", "1"),
        ("AWS_REGION", US_EAST_1),
        ("AWS_ACCESS_KEY_ID", "AKPSSVCSDATALAKE"),
        ("AWS_SECRET_ACCESS_KEY", "dummy_sk"),
    ]);
    build_client_from_env(
        env,
        &format!("http://{}", "us-east-1.s3-proxy.patsnap.info"),
    )
    .await
}

async fn build_liych_us_east_client() -> Client {
    let env = Env::from_slice(&[
        ("AWS_MAX_ATTEMPTS", "1"),
        ("AWS_REGION", US_EAST_1),
        ("AWS_ACCESS_KEY_ID", "AKPSPERSLIYCH"),
        ("AWS_SECRET_ACCESS_KEY", "dummy_sk"),
    ]);
    build_client_from_env(
        env,
        &format!("http://{}", "us-east-1.s3-proxy.patsnap.info"),
    )
    .await
}

async fn build_cjj_us_east_client() -> Client {
    let env = Env::from_slice(&[
        ("AWS_MAX_ATTEMPTS", "1"),
        ("AWS_REGION", US_EAST_1),
        ("AWS_ACCESS_KEY_ID", "caojinjuan"),
        ("AWS_SECRET_ACCESS_KEY", "dummy_sk"),
    ]);
    build_client_from_env(
        env,
        // &format!("http://{}", "s-ops-s3-proxy-us-aws.patsnap.info"),
        &format!("http://{}", DEV_PROXY_HOST),
    )
    .await
}

#[tokio::test]
async fn dt_us_east() {
    let output = build_dt_us_east_client()
        .await
        .get_object()
        .bucket("datalake-internal.patsnap.com")
        .key("dependencies.zip")
        .send()
        .await
        .unwrap();
    assert!(output.content_length() > 100);
    let output = build_dt_us_east_client()
        .await
        .list_objects()
        .bucket("datalake-internal.patsnap.com")
        .send()
        .await
        .unwrap();
    assert!(output.contents().unwrap().len() > 2);
}

#[tokio::test]
async fn lyc_us_east() {
    let output = build_liych_us_east_client()
        .await
        .get_object()
        .bucket("testpatsnapus")
        .key("liych/tmp/tidb_backup/2022-10-10--03/part-0-0")
        .send()
        .await
        .unwrap();
    assert!(output.content_length() > 10)
}

#[tokio::test]
async fn cjj_us_east() {
    let output = build_cjj_us_east_client()
        .await
        .get_object()
        .bucket("data-processing-data")
        .key("bigdata/caojinjuan/10k_x_patent_id.csv")
        .send()
        .await
        .unwrap();
    assert!(output.content_length() > 10)
}

#[tokio::test]
async fn wwt_test() {
    let env = Env::from_slice(&[
        ("AWS_MAX_ATTEMPTS", "1"),
        ("AWS_REGION", US_EAST_1),
        ("AWS_ACCESS_KEY_ID", "AKPSSVCSPROXYDEV"),
        ("AWS_SECRET_ACCESS_KEY", "dummy_sk"),
    ]);
    let client = build_client_from_env(
        env,
        &format!("http://{}", "us-east-1.s3-proxy.patsnap.info"),
        // &format!("http://{}", DEV_PROXY_HOST),
    )
    .await;
    let output = client
        .head_object()
        .bucket("autodeploy-patsnap-us-east-1")
        .key("aes_key.txt")
        .send()
        .await
        .unwrap();
    // assert!(output.content_length() > 1);
}

#[tokio::test]
async fn data_team_dev() {
    let env = Env::from_slice(&[
        ("AWS_MAX_ATTEMPTS", "1"),
        ("AWS_REGION", CN_NORTHWEST_1),
        ("AWS_ACCESS_KEY_ID", "AKPSTEAMDATA"),
        ("AWS_SECRET_ACCESS_KEY", "dummy_sk"),
    ]);
    let client = build_client_from_env(
        env,
        &format!("http://{}", "cn-northwest-1.s3-proxy.patsnap.info"),
        // &format!("http://{}", DEV_PROXY_HOST),
    )
    .await;

    let bucket = "datalake-internal.patsnap.com-cn-northwest-1";
    let key = "/tmp/cdc/ticdc/cn_source/legal/20221114/test_oplog_02.zip";
    let output = client
        .get_object()
        .bucket(bucket)
        .key(key)
        // .body(ByteStream::from_static("dummy".as_bytes()))
        .send()
        .await
        .unwrap();
    dbg!(output.e_tag());
}

#[tokio::test]
async fn cx() {
    let env = Env::from_slice(&[
        ("AWS_MAX_ATTEMPTS", "1"),
        ("AWS_REGION", US_EAST_1),
        ("AWS_ACCESS_KEY_ID", "AKPSSVCSDATA"),
        // ("AWS_ACCESS_KEY_ID", "AKPSSVCSPROXYDEV"),
        ("AWS_SECRET_ACCESS_KEY", "dummy_sk"),
    ]);
    let client = build_client_from_env(
        env,
        // &format!("http://{}", "us-east-1.s3-proxy.patsnap.info"),
        &format!("http://{}", DEV_PROXY_HOST),
    )
    .await;

    // 0066
    let bucket = "patsnap-general-source";
    let key = "pharmsnap/cde/slpzxx/20220718/CXHB2200111.json";
    match client.head_object().bucket(bucket).key(key).send().await {
        Ok(o) => {
            dbg!(o);
        }
        Err(e) => {
            dbg!(e);
        }
    };

    client
        .head_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn shf() {
    let env = Env::from_slice(&[
        ("AWS_MAX_ATTEMPTS", "1"),
        ("AWS_REGION", CN_NORTHWEST_1),
        ("AWS_ACCESS_KEY_ID", "AKPSPERS03SHF0Z"),
        ("AWS_SECRET_ACCESS_KEY", "dummy_sk"),
    ]);
    let client = build_client_from_env(
        env,
        &format!("http://{}", "cn-northwest-1.s3-proxy.patsnap.info"),
        // &format!("http://{}", DEV_PROXY_HOST),
    )
    .await;
    // datalake-internal.patsnap.com-cn-northwest-1.cn-northwest-1.s3-proxy.patsnap.info
    let output = client
        .get_object()
        .bucket("datalake-internal.patsnap.com-cn-northwest-1")
        .key("tmp/cdc/ticdc/cn_source/legal/20221115/oplog_1668480800004_cbcb_138.zip")
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn zx_new() {
    let env = Env::from_slice(&[
        ("AWS_MAX_ATTEMPTS", "1"),
        ("AWS_REGION", US_EAST_1),
        ("AWS_ACCESS_KEY_ID", "AKPSSVCS24DDATARDPROCESSINGBATCHQA"),
        ("AWS_SECRET_ACCESS_KEY", "dummy_sk"),
    ]);
    let client = build_client_from_env(
        env,
        // &format!("http://{}", "us-east-1.s3-proxy.patsnap.info"),
        &format!("http://{}", DEV_PROXY_HOST),
    )
    .await;
    let output = client
        .get_object()
        .bucket("datalake-internal.patsnap.com")
        .key("tmp/10w_pid.txt")
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn zx_old() {
    let env = Env::from_slice(&[
        ("AWS_MAX_ATTEMPTS", "1"),
        ("AWS_REGION", US_EAST_1),
        ("AWS_ACCESS_KEY_ID", "1AKPSSVCSDATA"),
        ("AWS_SECRET_ACCESS_KEY", "dummy_sk"),
    ]);
    let client = build_client_from_env(
        env,
        // &format!("http://{}", "us-east-1.s3-proxy.patsnap.info"),
        &format!("http://{}", DEV_PROXY_HOST),
    )
    .await;
    let output = client
        .get_object()
        .bucket("datalake-internal.patsnap.com")
        .key("tmp/10w_pid.txt")
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn tencent_list_buckets() {
    let env = Env::from_slice(&[
        ("AWS_MAX_ATTEMPTS", "1"),
        ("AWS_REGION", AP_SHANGHAI),
        // ("AWS_REGION", NA_ASHBURN),
        ("AWS_ACCESS_KEY_ID", "AKIDlT7kM0dGqOwS1Y4b7fjFkDdCospljYFm"),
        ("AWS_SECRET_ACCESS_KEY", ""),
    ]);
    let client = build_client_from_env(
        env,
        // &format!("http://{}", "cos.ap-shanghai.myqcloud.com"),
        // &format!("http://{}", "cos.na-ashburn.myqcloud.com"),
        &format!("http://{}", DEV_PROXY_HOST),
    )
    .await;

    // let output = client.list_buckets().send().await.unwrap();
    // output.buckets().unwrap().iter().for_each(|b| {
    //     dbg!(b.name().unwrap());
    // });
    // data-bio-source-cn-1251949819
    // data-bio-source-us-1251949819

    let objects = client
        .list_objects_v2()
        .bucket("data-bio-source-cn-1251949819")
        // .bucket("data-bio-source-us-1251949819")
        .send()
        .await
        .unwrap();
    dbg!(&objects.contents().unwrap());

    client
        .get_object()
        .bucket("data-bio-source-cn-1251949819")
        // .bucket("data-bio-source-us-1251949819")
        .key("asdsad")
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn opst() {
    let env = Env::from_slice(&[
        ("AWS_MAX_ATTEMPTS", "1"),
        ("AWS_REGION", AP_SHANGHAI),
        // ("AWS_REGION", NA_ASHBURN),
        ("AWS_ACCESS_KEY_ID", "AKIDlT7kM0dGqOwS1Y4b7fjFkDdCospljYFm"),
        ("AWS_SECRET_ACCESS_KEY", ""),
    ]);
    let client = build_client_from_env(
        env,
        // &format!("http://{}", "cos.ap-shanghai.myqcloud.com"),
        // &format!("http://{}", "cos.na-ashburn.myqcloud.com"),
        &format!("http://{}", DEV_PROXY_HOST),
    )
    .await;

    // let output = client.list_buckets().send().await.unwrap();
    // output.buckets().unwrap().iter().for_each(|b| {
    //     dbg!(b.name().unwrap());
    // });
    // data-bio-source-cn-1251949819
    // data-bio-source-us-1251949819

    let objects = client
        .list_objects_v2()
        .bucket("data-bio-source-cn-1251949819")
        // .bucket("data-bio-source-us-1251949819")
        .send()
        .await
        .unwrap();
    dbg!(&objects.contents().unwrap());

    client
        .get_object()
        .bucket("data-bio-source-cn-1251949819")
        // .bucket("data-bio-source-us-1251949819")
        .key("asdsad")
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn qwt() {
    let client = build_client_from_params(ClientParams {
        access_key: "AKPSPERS03QWT0Z",
        secret: "",
        region: NA_ASHBURN,
        endpoint: DEV_PROXY_ENDPOINT,
    });

    let objects = client
        .list_objects_v2()
        .bucket("patsnap-country-source-1251949819")
        .send()
        .await
        .unwrap();
    dbg!(&objects.contents().unwrap());

    let output = client
        .get_object()
        .bucket("patsnap-country-source-1251949819")
        .key("HK/A/12/51/79/0/output.json")
        .send()
        .await
        .unwrap();
    dbg!(&output.e_tag());
}

#[tokio::test]
async fn lrj() {
    let client = build_client_from_params(ClientParams {
        access_key: "AKPSSVCS14DDATADWCSCRIPT",
        secret: "",
        region: CN_NORTHWEST_1,
        endpoint: DEV_PROXY_ENDPOINT,
    });

    let objects = client
        .list_objects_v2()
        .bucket("patsnap-country-source-1251949819")
        .send()
        .await
        .unwrap();
    //
    // let output = client
    //     .get_object()
    //     .bucket("patsnap-country-source-1251949819")
    //     .key("HK/A/12/51/79/0/output.json")
    //     .send()
    //     .await
    //     .unwrap();
    // dbg!(&output.e_tag());
}

#[tokio::test]
async fn fxd() {
    let client = build_client_from_params(ClientParams {
        access_key: "AKPSPERS03FXD0Z",
        secret: "",
        region: CN_NORTHWEST_1,
        endpoint: DEV_PROXY_ENDPOINT,
    });

    client
        .get_object()
        .bucket("patsnap-country-source-1251949819")
        .key("HK/A/12/51/79/0/output.json")
        .send()
        .await
        .unwrap()
        .e_tag()
        .unwrap();
}

#[tokio::test]
async fn zsz() {
    let client = build_client_from_params(ClientParams {
        access_key: "AKPSPERS03ZSZ0Z",
        secret: "",
        region: CN_NORTHWEST_1,
        endpoint: DEV_PROXY_ENDPOINT,
    });

    client
        .get_object()
        .bucket("patsnap-country-source-1251949819")
        .key("HK/A/12/51/79/0/output.json")
        .send()
        .await
        .unwrap()
        .e_tag()
        .unwrap();
}

#[tokio::test]
async fn test_9554() {
    let client = build_client_from_params(ClientParams {
        access_key: "AKPSSVCS04OPST",
        secret: "",
        region: CN_NORTHWEST_1,
        endpoint: DEV_PROXY_ENDPOINT,
    });

    let objects = client
        .get_object()
        .bucket("anniversary")
        .key("image/birthday_bottom.jpg")
        .send()
        .await
        .unwrap();
    objects.e_tag().unwrap();
}

#[tokio::test]
async fn test_list_buckets() {
    // "ap-shanghai" => Ok("cos.ap-shanghai.myqcloud.com"),
    // "na-ashburn" => Ok("cos.na-ashburn.myqcloud.com"),
    let client = build_client_from_params(ClientParams {
        access_key: "AKIDlT7kM0dGqOwS1Y4b7fjFkDdCospljYFm",
        secret: "",
        region: NA_ASHBURN,
        endpoint: "http://cos.na-ashburn.myqcloud.com",
    });

    let output = client.list_buckets().send().await.unwrap();
    dbg!(output.buckets().unwrap().len());
}
