use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use http::{Response, StatusCode};
use hyper::{client::HttpConnector, Body};
use log::debug;
use piam_proxy_core::{
    account::aws::AwsAccount,
    config::POLICY_MODEL,
    container::{IamContainer, PolicyQueryParams},
    error::{ProxyError, ProxyResult},
    input::Input,
    policy::FindEffect,
    request::{forward, ApplyResult, HttpRequestExt},
    response::HttpResponseExt,
    signature::{
        aws::{AwsSigv4, AwsSigv4SignParams},
        split_to_base_and_account_code,
    },
    state::{ArcState, ProxyState},
    type_alias::{HttpClient, HttpRequest, HttpResponse},
};
use piam_tracing::logger::change_debug;

use crate::{
    config,
    config::SERVICE,
    parser::{ActionKind, S3Input},
    policy::S3Statement,
    request::S3RequestTransform,
    S3Config,
};

pub type S3ProxyState = ArcState<S3Statement, S3Config>;

pub async fn health() -> impl IntoResponse {
    "OK"
}

pub async fn manage(
    State(state): State<S3ProxyState>,
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
        let on = change_debug(state.load().log_handle.as_ref().unwrap(), debug.as_str());
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
) -> ProxyResult<HttpResponse> {
    let proxy_hosts: &Vec<String> = &state.load().extended_config.proxy_hosts;
    req.adapt_path_style(path, proxy_hosts);
    handle(State(state), req).await
}

pub async fn handle(
    State(state): State<S3ProxyState>,
    req: HttpRequest,
) -> ProxyResult<HttpResponse> {
    debug!("req.uri '{}'", req.uri());
    debug!("req.method {}", req.method());
    debug!("req.headers {:#?}", req.headers());
    let state = state.load();

    // Get input structure by parsing the request for specific protocol.
    // Example: getting S3Input with bucket and key as its fields.
    let s3_config = &state.extended_config;
    let input = S3Input::from_http(&req, Some(s3_config))?;

    let iam_container = &state.iam_container;

    // aws sigv4 specific
    let (access_key, region) = req.extract_aws_access_key_and_region()?;
    // When feature uni-key is enabled, base_access_key is aws access_key,
    // otherwise base_access_key + account_code = aws_access_key
    #[cfg(feature = "uni-key")]
    let (account, region, base_access_key) = {
        let access_info = s3_config
            .get_uni_key_info()?
            .find_access_info_input(&input)?;
        (&access_info.account, &access_info.region, access_key)
    };
    #[cfg(not(feature = "uni-key"))]
    let (account, base_access_key) = {
        let (base_access_key, code) = split_to_base_and_account_code(access_key)?;
        let account = iam_container.find_account_by_code(code)?;
        (account, base_access_key)
    };

    // Find matching policies by base_access_key in the request
    let user = iam_container.find_user_by_base_access_key(base_access_key)?;
    let groups = iam_container.find_groups_by_user(user)?;
    let policy_query_param = PolicyQueryParams {
        roles: None,
        user: None,
        groups: Some(groups),
        account,
        region,
    };
    let policies = iam_container.find_policies(&policy_query_param)?;

    // Find effects in policies by input structure
    let effects = policies.find_effects_by_input(&input)?;

    // Apply effects to the request and return the final response
    let region = region.to_string();
    let res = match req.apply_effects(effects)? {
        ApplyResult::Forward(mut raw_req) => {
            raw_req.set_actual_host(s3_config, &region)?;
            let aws_sigv4_sign_params = AwsSigv4SignParams {
                account,
                service: SERVICE,
                region: &region,
            };
            let signed_req = raw_req
                .sign_with_aws_sigv4_params(&aws_sigv4_sign_params)
                .await
                .expect("sign request error");
            forward(signed_req, &state.http_client).await?
        }
        ApplyResult::Reject(response) => response,
    };
    // add tracing info
    Ok(res.add_piam_headers_with_random_id())
}
