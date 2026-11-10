mod common;
use common::*;
use serde_json::json;

#[tokio::test]
async fn fr7_health_returns_ok() {
    let (status, body) = get("/health").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, json!({ "status": "ok" }));
}
