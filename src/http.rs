//! HTTP layer: routing, DTOs and error mapping (ADR-0001, ADR-0005).

use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

/// Builds the application router. Used by `main` and by integration tests.
pub fn app() -> Router {
    Router::new().route("/health", get(health))
}

/// FR-7: liveness check used by Railway.
async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
