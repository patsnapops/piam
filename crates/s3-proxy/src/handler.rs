use std::{collections::HashMap, net::SocketAddr};

use axum::{
    extract::{ConnectInfo, Path, Query, State},
    response::IntoResponse,
};
use busylib::logger::change_debug;
use http::{Response, StatusCode};
use hyper::Body;
use log::debug;
use piam_object_storage::{input::ObjectStorageInput, policy::ObjectStoragePolicy};
use piam_proxy_core::{
    condition::input::Condition,
    container::PolicyFilterParams,
    error::{eok_ctx, ProxyResult},
    policy::FindEffect,
    request::{forward, HttpRequestExt},
    response::HttpResponseExt,
    signature::aws::{AwsSigv4, AwsSigv4SignParams},
    state::ArcState,
    type_alias::{HttpRequest, HttpResponse},
};

use crate::{config::SERVICE, request::S3RequestTransform, S3Config};

pub type S3ProxyState = ArcState<ObjectStoragePolicy, S3Config>;

pub async fn health() -> impl IntoResponse {
    "OK"
}

pub async fn manage(
    State(state): State<S3ProxyState>,
    Query(params): Query<HashMap<String, String>>,
    // mut req: Request<Body>,
) -> HttpResponse {
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
    Path(path): Path<String>,
    State(state): State<S3ProxyState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    mut req: HttpRequest,
) -> ProxyResult<HttpResponse> {
    let proxy_hosts = &state.load().extended_config.proxy_hosts.domains;
    req.adapt_path_style(path, proxy_hosts);
    handle(State(state), ConnectInfo(addr), req).await
}

pub async fn handle(
    State(state): State<S3ProxyState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: HttpRequest,
) -> ProxyResult<HttpResponse> {
    debug!("addr {}", addr);
    debug!("req.uri '{}'", req.uri());
    debug!("req.method {}", req.method());
    debug!("req.headers {:#?}", req.headers());

    let state = state.load();

    // Get input structure by parsing the request for specific protocol.
    // Example: getting S3Input with bucket and key as its fields.
    let s3_config = &state.extended_config;
    let input = ObjectStorageInput::from_s3(&req, &s3_config.proxy_hosts)?;

    let iam_container = &state.iam_container;

    // aws sigv4 specific
    #[allow(unused)]
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
    let policy_filter_param = PolicyFilterParams::new_with_default(groups, account, region);
    let policies = iam_container.find_policies(&policy_filter_param)?;

    // Apply conditional effects that finding in policies
    let condition = Condition::new_with_addr(addr);
    let condition_effects = policies.condition.find_effects(&condition)?;
    let req = req.apply_effects(condition_effects)?;
    // Apply user input effects that finding in policies
    let user_input_effects = policies.user_input.find_effects(&input)?;
    let mut raw_req = req.apply_effects(user_input_effects)?;

    // Sign and forward
    raw_req.set_actual_host(s3_config, region)?;
    let sign_params = AwsSigv4SignParams::new_with(account, SERVICE, region);
    let signed_req = eok_ctx(
        raw_req.sign_with_aws_sigv4_params(&sign_params).await,
        "sign_with_aws_sigv4_params in handler",
    );
    let res = forward(signed_req, &state.http_client).await?;

    // add tracing info
    Ok(res.add_piam_headers_with_random_id())
}
