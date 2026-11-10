mod common;
use common::*;
use serde_json::json;

#[tokio::test]
async fn fr7_health_returns_ok() {
    let (status, body) = get("/health").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, json!({ "status": "ok" }));
}

// ---- FR-1 / FR-2 / FR-3: successful calculations (AC-1, AC-2, AC-3) ----

#[tokio::test]
async fn ac1_point_one_plus_point_two_is_point_three() {
    let (status, body) = calc(json!({ "op": "add", "a": "0.1", "b": "0.2" })).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, json!({ "result": "0.3" }));
}

#[tokio::test]
async fn ac2_every_prd_example() {
    let cases = [
        ("add", "0.1", Some("0.2"), "0.3"),
        ("subtract", "5", Some("7.5"), "-2.5"),
        ("multiply", "1.5", Some("4"), "6"),
        ("divide", "1", Some("3"), "0.3333333333333333333333333333"),
        ("modulo", "10", Some("3"), "1"),
        ("power", "2", Some("10"), "1024"),
        ("percent", "15", Some("200"), "30"),
        ("sqrt", "16", None, "4"),
    ];
    for (op, a, b, expected) in cases {
        let mut req = json!({ "op": op, "a": a });
        if let Some(b) = b {
            req["b"] = json!(b);
        }
        let (status, body) = calc(req).await;
        assert_eq!(status, StatusCode::OK, "{op}");
        assert_eq!(body, json!({ "result": expected }), "{op}");
    }
}

// ---- FR-5: every row of the error table (AC-5, AC-10) ----

async fn assert_error(req: serde_json::Value, status: StatusCode, code: &str) {
    let (got_status, body) = calc(req.clone()).await;
    assert_eq!(got_status, status, "{req}");
    assert_eq!(body["error"]["code"], code, "{req}");
    assert!(body["error"]["message"].is_string(), "{req}");
}

#[tokio::test]
async fn fr5_invalid_request_shapes() {
    let bad = StatusCode::BAD_REQUEST;
    assert_error(json!({ "a": "1", "b": "2" }), bad, "INVALID_REQUEST").await;
    assert_error(json!({ "op": "add", "b": "2" }), bad, "INVALID_REQUEST").await;
    assert_error(json!({ "op": "add", "a": "1" }), bad, "INVALID_REQUEST").await;
    assert_error(
        json!({ "op": "sqrt", "a": "4", "b": "1" }),
        bad,
        "INVALID_REQUEST",
    )
    .await;
    assert_error(
        json!({ "op": "add", "a": "1", "b": "2", "c": "3" }),
        bad,
        "INVALID_REQUEST",
    )
    .await;
    assert_error(
        json!({ "op": "add", "a": 0.1, "b": "2" }),
        bad,
        "INVALID_REQUEST",
    )
    .await;
    assert_error(json!(["add", "1", "2"]), bad, "INVALID_REQUEST").await;
}

#[tokio::test]
async fn fr5_malformed_json_is_invalid_request() {
    let (status, body) = post_raw("/v1/calculate", "{not json").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "INVALID_REQUEST");
}

#[tokio::test]
async fn fr5_unknown_operation() {
    let req = json!({ "op": "log", "a": "1", "b": "2" });
    assert_error(req, StatusCode::BAD_REQUEST, "UNKNOWN_OPERATION").await;
}

#[tokio::test]
async fn fr5_invalid_numbers() {
    for a in ["abc", "1e3", "", " 1", "0.00000000000000000000000000001"] {
        let req = json!({ "op": "add", "a": a, "b": "1" });
        assert_error(req, StatusCode::BAD_REQUEST, "INVALID_NUMBER").await;
    }
}

#[tokio::test]
async fn fr5_unprocessable_calculations() {
    let u = StatusCode::UNPROCESSABLE_ENTITY;
    assert_error(
        json!({ "op": "divide", "a": "1", "b": "0" }),
        u,
        "DIVISION_BY_ZERO",
    )
    .await;
    assert_error(
        json!({ "op": "modulo", "a": "1", "b": "0" }),
        u,
        "DIVISION_BY_ZERO",
    )
    .await;
    assert_error(json!({ "op": "sqrt", "a": "-1" }), u, "NEGATIVE_SQRT").await;
    assert_error(
        json!({ "op": "power", "a": "2", "b": "0.5" }),
        u,
        "UNSUPPORTED_EXPONENT",
    )
    .await;
    let max = "79228162514264337593543950335";
    assert_error(json!({ "op": "add", "a": max, "b": "1" }), u, "OVERFLOW").await;
}

#[tokio::test]
async fn nfr2_body_over_16kb_is_rejected() {
    let padding = "0".repeat(17 * 1024);
    let body = format!(r#"{{"op":"add","a":"{padding}1","b":"1"}}"#);
    let (status, body) = post_raw("/v1/calculate", body).await;
    assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE);
    assert_eq!(body["error"]["code"], "PAYLOAD_TOO_LARGE");
}

#[tokio::test]
async fn unknown_route_uses_error_shape() {
    let (status, body) = get("/v1/nope").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"]["code"], "NOT_FOUND");
}

#[tokio::test]
async fn wrong_method_uses_error_shape() {
    let (status, body) = get("/v1/calculate").await;
    assert_eq!(status, StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(body["error"]["code"], "METHOD_NOT_ALLOWED");
}
