use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use http::{Response, StatusCode};
use hyper::Body;
use log::debug;
use piam_proxy_core::{
    config::CORE_CONFIG,
    error::ProxyResult,
    input::Input,
    request::{find_effect, find_policies_by_access_key, HttpRequestExt},
    response::HttpResponseExt,
    sign::{sign_with_amz_params, AmzExt},
    state::SharedState,
    type_alias::{ApplyResult, HttpRequest, HttpResponse},
};
use piam_tracing::logger::change_debug;

use crate::{parser::S3Input, policy::S3PolicyStatementImpl, request::S3RequestTransform};

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
) -> ProxyResult<HttpResponse> {
    req.adapt_path_style(path);
    handle(State(state), req).await
}

pub async fn handle(
    State(state): State<S3ProxyState>,
    req: HttpRequest,
) -> ProxyResult<HttpResponse> {
    debug!("handle {:#?}", req);
    let access_key = req.extract_access_key()?;
    let lock = state.read().await;
    let policies = find_policies_by_access_key(
        access_key,
        &lock.principal_container,
        &lock.policy_container,
    )?;
    let input = S3Input::from_http(&req).expect("parse input should not fail");
    let effect = find_effect(policies, &input)?;
    let apply_result = req.apply_effect(effect);

    let res: HttpResponse = match apply_result {
        ApplyResult::Forward(mut new_req) => {
            new_req.set_actual_host();
            // TODO now: 2 support multi cloud sign
            // 1. find bucket belong to which cloud
            // 2. sign_with_xxx_params
            new_req = sign_with_amz_params(new_req)
                .await
                .expect("sign should not fail");
            let client = &CORE_CONFIG.load().client;
            debug!("new_req {:#?}", new_req);
            client
                .request(new_req)
                .await
                .expect("request to s3 service should not fail")
        }
        ApplyResult::Reject(response) => response,
    };
    // add tracing info
    Ok(res.add_piam_headers_with_random_id())
}
