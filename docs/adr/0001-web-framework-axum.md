# 0001. Use axum as the HTTP framework

- Status: Accepted
- Date: 2026-11-10
- Deciders: Serban
- Related requirements: FR-1, FR-6, FR-7, NFR-2, NFR-5

## Context
The owner chose Rust. We need an HTTP framework that handles JSON, body size limits and graceful errors, and that
works with OpenAPI generation (FR-6). The service is tiny, so ecosystem fit and simplicity matter more than raw speed.

## Options considered
### axum
- Built on tokio, hyper and tower; maintained by the tokio team.
- Tower middleware gives body limits (`RequestBodyLimitLayer`) and tracing without custom code.
- First-class support in `utoipa`; routers are testable in-process with `tower::ServiceExt::oneshot` (no network).
- Con: extractor errors need mapping to our error shape.
### actix-web
- Very fast and mature. Con: its own runtime and middleware model; less shared ecosystem with tower.
### rocket
- Ergonomic macros. Con: slower release cadence, heavier magic, smaller middleware ecosystem.

## Decision
Use **axum** with tokio.

## Consequences
- Body limit, timeouts and tracing come from tower layers.
- We must map axum's JSON rejection errors to `INVALID_REQUEST` (FR-5) ourselves.
- Integration tests call the router directly, so they are fast and need no port.
