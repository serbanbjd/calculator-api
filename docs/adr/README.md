# Architecture Decision Records

One decision per file, written with the `writing-adr` skill. Accepted ADRs are not edited: a new ADR supersedes them.

| # | Decision | Status |
|---|---|---|
| [0001](0001-web-framework-axum.md) | Use axum as the HTTP framework | Accepted |
| [0002](0002-exact-decimal-arithmetic.md) | Use `rust_decimal` for all arithmetic | Accepted |
| [0003](0003-single-calculate-endpoint.md) | One `POST /v1/calculate` endpoint with an `op` field | Accepted |
| [0004](0004-openapi-generated-from-code.md) | Generate OpenAPI from code with utoipa, serve Swagger UI | Accepted |
| [0005](0005-single-crate-domain-http-split.md) | Single crate, pure domain module, thin HTTP module | Accepted |
| [0006](0006-container-and-railway-deploy.md) | Multi-stage Docker image, deployed on Railway | Accepted |
| [0007](0007-ci-on-github-actions.md) | CI on GitHub Actions for every PR | Accepted |
