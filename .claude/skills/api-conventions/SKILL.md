---
name: api-conventions
description: Use when writing, changing or reviewing any HTTP endpoint of the calculator API. Defines request/response shape, decimal handling, errors, validation and testing conventions.
---

# API conventions

These are defaults. If the PRD or an accepted ADR says otherwise, the document wins: update this skill to match.

## Requests and responses

- JSON in, JSON out, `Content-Type: application/json`.
- Paths are versioned: `/v1/...`. Health check at `GET /health` (unversioned).
- Every endpoint is documented in the OpenAPI spec (generated from code with `utoipa`), served at
  `GET /openapi.json`, with Swagger UI at `GET /docs`. An endpoint missing from the spec is a review blocker.
- Field names are `snake_case`.
- **Numbers are sent and returned as JSON strings** (`"0.1"`), parsed into `rust_decimal::Decimal`.
  This avoids float rounding in JSON parsers on both sides. Never use `f64` for a value the user sees.
- A successful result looks like: `{ "result": "0.3" }`.

## Errors

- One error shape everywhere:
  `{ "error": { "code": "DIVISION_BY_ZERO", "message": "Cannot divide by zero" } }`
- Codes are stable `UPPER_SNAKE_CASE` strings; messages are for humans and may change.
- `400` malformed JSON or invalid number, `422` valid input that cannot be computed
  (division by zero, overflow), `404` unknown route, `413` body too large, `500` never on purpose.
- Never leak internal details (panics, stack traces, library error text) in a response.

## Validation

- Validate at the boundary (the HTTP handler); the domain layer receives only valid types.
- Reject, don't guess: no silent truncation, no implicit rounding beyond what the PRD specifies.
- Limit request body size.

## Code layout

- The domain logic (pure functions over `Decimal`) has no axum dependency, so it is testable alone.
- Handlers are thin: parse, call domain, map result or error to HTTP.
- No `unwrap()`/`expect()` on user input paths.

## Tests

- Unit tests next to the domain code for every operation and edge case.
- Integration tests in `tests/` that call the router over HTTP (no network needed).
- Each test names the requirement it covers, for example `fr2_division_by_zero_returns_422`.
