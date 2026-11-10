# 0003. One POST /v1/calculate endpoint with an op field

- Status: Accepted
- Date: 2026-11-10
- Deciders: Serban
- Related requirements: FR-1, FR-2, FR-3, FR-5, FR-6

## Context
The API exposes eight operations, one per call, to any client: a UI, `curl` or an AI agent using it as a tool.
Operands are decimal strings.

## Options considered
### A single `POST /v1/calculate` with `{ "op", "a", "b" }`
- One request and response schema; adding an operation means adding an enum value.
- Easy to describe as a single tool for AI agents; one place for validation and error mapping.
- Con: unary vs binary operands are validated in code, not by URL shape.
### One endpoint per operation (`POST /v1/add`, ...)
- Very explicit URLs, per-operation schemas. Con: eight near-identical handlers and OpenAPI entries.
### `GET /v1/add?a=0.1&b=0.2`
- Cacheable, easy in a browser. Con: numbers in query strings invite encoding mistakes (`+`, spaces);
  `GET` for computation is fine but gives no body for future fields.

## Decision
A single **`POST /v1/calculate`** with a tagged `op` enum. Operations: `add`, `subtract`, `multiply`, `divide`,
`modulo`, `power`, `percent` (need `a` and `b`) and `sqrt` (needs `a` only; `b` is rejected).
Version in the path (`/v1`) so a breaking change can live beside the old one.

## Consequences
- Request validation (unknown fields, missing or extra operand) lives in one handler.
- OpenAPI shows one schema with an `op` enum, documented with an example per operation.
