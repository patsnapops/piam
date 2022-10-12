use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use http::{HeaderValue, Response, StatusCode};
use hyper::Body;
use log::debug;
use piam_proxy_core::{
    config::CORE_CONFIG,
    request::HttpRequestExt,
    sign::sign_with_amz_params,
    state::SharedState,
    type_alias::{ApplyResult, HttpRequest, HttpResponse},
};
use piam_tracing::logger::change_debug;
use uuid::Uuid;

use crate::{policy::S3PolicyStatementImpl, request::S3RequestTransform};

pub type S3ProxyState = SharedState<S3PolicyStatementImpl>;

pub async fn health() -> impl IntoResponse {
    "OK"
}

pub async fn manage(
    Query(params): Query<HashMap<String, String>>,
    // mut req: Request<Body>,
) -> Response<Body> {
    // TODO: turn debug mode on/off
    fn resp(payload: &str) -> HttpResponse {
        Response::builder()
            .body(Body::from(payload.to_string()))
            .unwrap()
    }
    if let Some(debug) = params.get("debug") {
        let on = change_debug(
            CORE_CONFIG.load().log_handle.as_ref().unwrap(),
            debug.as_str(),
        );
        return if on {
            resp("debug mode on")
        } else {
            resp("debug mode off")
        };
    }
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from("invalid request"))
        .unwrap()
}

pub async fn handle_path(
    State(state): State<S3ProxyState>,
    Path(path): Path<String>,
    mut req: HttpRequest,
) -> HttpResponse {
    req.adapt_path_style(path);
    handle(State(state), req).await
}

pub async fn handle(State(state): State<S3ProxyState>, req: HttpRequest) -> HttpResponse {
    debug!("handle req{:#?}", req);
    let id = Uuid::new_v4().to_string();

    let lock = state.read().await;
    let result = req.apply_policies(&lock.principal_container, &lock.policy_container);
    let mut res = match result {
        ApplyResult::Forward(mut new_req) => {
            new_req.set_actual_host();
            // TODO now: 2 support multi cloud sign
            // 1. find bucket belong to which cloud
            // 2. sign_with_xxx_params
            new_req = sign_with_amz_params(new_req).await.unwrap();
            let client = &CORE_CONFIG.load().client;
            debug!("new_req {:#?}", new_req);
            client.request(new_req).await.unwrap()
        }
        ApplyResult::Reject(response) => response,
    };

    // add tracing info
    add_piam_headers(&mut res, id);
    res
}

fn add_piam_headers(res: &mut Response<Body>, id: String) {
    let headers = res.headers_mut();
    headers.append(
        "x-patsnap-proxy-type",
        HeaderValue::from_static("Patsnap S3 Proxy"),
    );
    headers.append("x-patsnap-request-id", HeaderValue::from_str(&id).unwrap());
}

#[cfg(test)]
#[cfg(feature = "integration-test")]
mod tests {
    use arc_swap::access::Access;
    use aws_config::{from_env, provider_config::ProviderConfig};
    use aws_sdk_s3::{
        error::HeadBucketErrorKind,
        model::{CompletedMultipartUpload, CompletedPart, Object},
        types::{ByteStream, SdkError},
        Client, Endpoint,
    };
    use aws_smithy_client::{erase::DynConnector, never::NeverConnector};
    use aws_types::os_shim_internal::Env;
    use futures::future;
    use uuid::Uuid;

    use crate::{
        config::{DEV_PROXY_HOST, S3_CONFIG},
        S3Config,
    };

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
    async fn list_objects_v1() {
        let output = build_client()
            .await
            .list_objects()
            .bucket("anniversary")
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
        put_object_random_key("qa-ops-test-s3").await;
        // fixme: support special char like *
        // put_object_with_key("中").await;
        // put_object_with_key("*").await;
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
            ("AWS_REGION", "cn-northwest-1"),
            ("AWS_ACCESS_KEY_ID", REAL_ACCESS_KEY_ID),
            ("AWS_SECRET_ACCESS_KEY", REAL_SECRET_ACCESS_KEY),
        ]);
        build_client_with_params(env, "http://s3.cn-northwest-1.amazonaws.com.cn").await
    }

    async fn build_fake_key_to_us_east_client_dev() -> Client {
        let env = Env::from_slice(&[
            ("AWS_MAX_ATTEMPTS", "1"),
            ("AWS_REGION", "us-east-1"),
            ("AWS_ACCESS_KEY_ID", "AKPSSVCSDATALAKE"),
            ("AWS_SECRET_ACCESS_KEY", "dummy_sk"),
        ]);
        build_client_with_params(env, &format!("http://{}", DEV_PROXY_HOST)).await
    }

    async fn build_fake_key_to_cn_northwest_client_dev() -> Client {
        let env = Env::from_slice(&[
            ("AWS_MAX_ATTEMPTS", "1"),
            ("AWS_REGION", "cn-northwest-1"),
            ("AWS_ACCESS_KEY_ID", "AKPSSVCSPROXYDEV"),
            ("AWS_SECRET_ACCESS_KEY", "dummy_sk"),
        ]);
        build_client_with_params(env, &format!("http://{}", DEV_PROXY_HOST)).await
    }

    async fn build_client_with_params(env: Env, endpoint: &str) -> Client {
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

    async fn build_dt_us_east_client() -> Client {
        let env = Env::from_slice(&[
            ("AWS_MAX_ATTEMPTS", "1"),
            ("AWS_REGION", "us-east-1"),
            ("AWS_ACCESS_KEY_ID", "AKPSSVCSDATALAKE"),
            ("AWS_SECRET_ACCESS_KEY", "dummy_sk"),
        ]);
        build_client_with_params(
            env,
            &format!("http://{}", "s-ops-s3-proxy-us-aws.patsnap.info"),
        )
        .await
    }

    async fn build_liych_us_east_client() -> Client {
        let env = Env::from_slice(&[
            ("AWS_MAX_ATTEMPTS", "1"),
            ("AWS_REGION", "us-east-1"),
            ("AWS_ACCESS_KEY_ID", "AKPSPERSLIYCH"),
            ("AWS_SECRET_ACCESS_KEY", "dummy_sk"),
        ]);
        build_client_with_params(
            env,
            &format!("http://{}", "s-ops-s3-proxy-us-aws.patsnap.info"),
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
    async fn lyc_us_east_get_object() {
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
}
