# 0005. Single crate with a pure domain module and a thin HTTP module

- Status: Accepted
- Date: 2026-11-10
- Deciders: Serban
- Related requirements: FR-2 to FR-5, NFR-5

## Context
The arithmetic rules are the valuable, risky part; HTTP is plumbing. We want the rules testable without a server,
but the project is too small to justify multiple crates.

## Options considered
### One crate: `domain` (pure functions over `Decimal`) + `http` (axum handlers, DTOs, error mapping) + `main.rs`
- Clear dependency direction (`http` → `domain`, never back). Fast to build. Con: the boundary is a convention, not enforced by the compiler.
### Cargo workspace with `calc-core` and `calc-api` crates
- Compiler-enforced boundary. Con: more files and build config for ~300 lines of code.
### Everything in handlers
- Least code. Con: rules tangled with HTTP, harder to unit test.

## Decision
**One crate**, modules `domain`, `http`, `config`; `lib.rs` exposes `app()` so integration tests can build the router.
`domain` must not import axum or serde_json.

## Consequences
- Unit tests live in `domain`; HTTP integration tests in `tests/`.
- If the domain is ever reused (CLI, WASM), extract it into its own crate then.
