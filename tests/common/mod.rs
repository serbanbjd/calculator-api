#![allow(dead_code)]
use axum::{body::Body, http::Request};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

pub use axum::http::StatusCode;

/// Sends a request to the in-process router and returns status + JSON body.
pub async fn send(req: Request<Body>) -> (StatusCode, Value) {
    let res = calculator_api::app()
        .oneshot(req)
        .await
        .expect("router error");
    let status = res.status();
    let bytes = res.into_body().collect().await.expect("body").to_bytes();
    let body = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, body)
}

pub async fn get(path: &str) -> (StatusCode, Value) {
    send(Request::get(path).body(Body::empty()).unwrap()).await
}

pub async fn post_raw(path: &str, body: impl Into<Body>) -> (StatusCode, Value) {
    send(
        Request::post(path)
            .header("content-type", "application/json")
            .body(body.into())
            .unwrap(),
    )
    .await
}

pub async fn calc(body: Value) -> (StatusCode, Value) {
    post_raw("/v1/calculate", body.to_string()).await
}
