//! HTTP layer: routing, DTOs and error mapping (ADR-0001, ADR-0003, ADR-0005).
//! Handlers only parse, call `domain` and map the outcome to HTTP.

use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::domain::{self, CalcError, Operation};

/// NFR-2: request bodies above this size are rejected with 413.
pub const MAX_BODY_BYTES: usize = 16 * 1024;

/// Builds the application router. Used by `main` and by integration tests.
pub fn app() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/v1/calculate", post(calculate))
        .fallback(not_found)
        .method_not_allowed_fallback(method_not_allowed)
        .layer(DefaultBodyLimit::max(MAX_BODY_BYTES))
}

/// FR-7: liveness check used by Railway.
async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CalculateRequest {
    /// Operation name; validated in the handler so an unknown value gets its own error code.
    pub op: String,
    pub a: String,
    #[serde(default)]
    pub b: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CalculateResponse {
    pub result: String,
}

/// FR-1: compute one operation.
async fn calculate(
    body: Result<Json<serde_json::Map<String, Value>>, JsonRejection>,
) -> Result<Json<CalculateResponse>, ApiError> {
    let Json(object) = body.map_err(ApiError::from_rejection)?;
    // Going through a JSON object first: serde would otherwise also accept
    // a positional array like ["add","1","2"] for this struct.
    let req: CalculateRequest =
        serde_json::from_value(Value::Object(object)).map_err(|_| ApiError::INVALID_REQUEST)?;
    let op = parse_operation(&req.op).ok_or(ApiError::UNKNOWN_OPERATION)?;
    let a = domain::parse_number(&req.a)?;
    let b = req.b.as_deref().map(domain::parse_number).transpose()?;
    let result = domain::calculate(op, a, b)?;
    Ok(Json(CalculateResponse {
        result: result.to_string(),
    }))
}

fn parse_operation(s: &str) -> Option<Operation> {
    use Operation::*;
    Some(match s {
        "add" => Add,
        "subtract" => Subtract,
        "multiply" => Multiply,
        "divide" => Divide,
        "modulo" => Modulo,
        "power" => Power,
        "percent" => Percent,
        "sqrt" => Sqrt,
        _ => return None,
    })
}

async fn not_found() -> ApiError {
    ApiError::new(StatusCode::NOT_FOUND, "NOT_FOUND", "No such route")
}

async fn method_not_allowed() -> ApiError {
    ApiError::new(
        StatusCode::METHOD_NOT_ALLOWED,
        "METHOD_NOT_ALLOWED",
        "Method not allowed on this route",
    )
}

/// FR-5: the one error shape, `{"error":{"code","message"}}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApiError {
    pub status: StatusCode,
    pub code: &'static str,
    pub message: &'static str,
}

impl ApiError {
    const fn new(status: StatusCode, code: &'static str, message: &'static str) -> Self {
        Self {
            status,
            code,
            message,
        }
    }

    const INVALID_REQUEST: Self = Self::new(
        StatusCode::BAD_REQUEST,
        "INVALID_REQUEST",
        r#"Body must be JSON like {"op":"add","a":"1","b":"2"}, with numbers as strings"#,
    );
    const UNKNOWN_OPERATION: Self = Self::new(
        StatusCode::BAD_REQUEST,
        "UNKNOWN_OPERATION",
        "op must be one of: add, subtract, multiply, divide, modulo, power, percent, sqrt",
    );
    const PAYLOAD_TOO_LARGE: Self = Self::new(
        StatusCode::PAYLOAD_TOO_LARGE,
        "PAYLOAD_TOO_LARGE",
        "Request body is larger than 16 KB",
    );

    /// Never echoes library error text back to the client (api-conventions).
    fn from_rejection(rejection: JsonRejection) -> Self {
        if rejection.status() == StatusCode::PAYLOAD_TOO_LARGE {
            Self::PAYLOAD_TOO_LARGE
        } else {
            Self::INVALID_REQUEST
        }
    }
}

impl From<CalcError> for ApiError {
    fn from(err: CalcError) -> Self {
        use CalcError::*;
        let unprocessable = StatusCode::UNPROCESSABLE_ENTITY;
        match err {
            InvalidNumber => Self::new(
                StatusCode::BAD_REQUEST,
                "INVALID_NUMBER",
                "Operands must be plain decimal strings like \"-12.5\", with at most 28 significant digits",
            ),
            MissingOperand => Self::new(
                StatusCode::BAD_REQUEST,
                "INVALID_REQUEST",
                "This operation needs both a and b",
            ),
            UnexpectedOperand => Self::new(
                StatusCode::BAD_REQUEST,
                "INVALID_REQUEST",
                "This operation takes only a; remove b",
            ),
            DivisionByZero => Self::new(unprocessable, "DIVISION_BY_ZERO", "Cannot divide by zero"),
            NegativeSqrt => Self::new(
                unprocessable,
                "NEGATIVE_SQRT",
                "Cannot take the square root of a negative number",
            ),
            UnsupportedExponent => Self::new(
                unprocessable,
                "UNSUPPORTED_EXPONENT",
                "power only supports integer exponents",
            ),
            Overflow => Self::new(
                unprocessable,
                "OVERFLOW",
                "Result is too large to represent",
            ),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = json!({ "error": { "code": self.code, "message": self.message } });
        (self.status, Json(body)).into_response()
    }
}
