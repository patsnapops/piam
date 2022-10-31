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
    debug!("handle {:#?}", req);
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
