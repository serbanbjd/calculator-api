# 0004. Generate OpenAPI from code with utoipa and serve Swagger UI

- Status: Accepted
- Date: 2026-11-10
- Deciders: Serban
- Related requirements: FR-6

## Context
The owner wants Swagger UI at `/docs`. The spec must stay accurate as the code changes, or Swagger UI misleads its users,
including AI agents that read `/openapi.json` to learn the tool.

## Options considered
### Generate from code with `utoipa` and serve UI with `utoipa-swagger-ui`
- Spec derived from the real request/response types: it cannot silently drift.
- Con: derive macros on types; Swagger UI assets are embedded in the binary (larger image, a few MB).
### Hand-written `openapi.yaml`
- Full control, spec-first. Con: drifts from code unless a contract test checks it.
### Serve the spec only, no UI
- Smallest. Con: does not meet FR-6.

## Decision
Generate with **utoipa**; serve `/openapi.json` and Swagger UI at **`/docs`** with **utoipa-swagger-ui**.

## Consequences
- Every handler and DTO carries `#[utoipa::path]` / `ToSchema`; `pr-review` treats a missing one as a blocker.
- A test asserts `/openapi.json` contains `/v1/calculate` and `/health` (AC-6).
- utoipa-swagger-ui downloads UI assets at build time; the Docker build needs network access (fine on Railway).
