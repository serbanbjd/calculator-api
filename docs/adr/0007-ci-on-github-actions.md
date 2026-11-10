# 0007. CI on GitHub Actions for every pull request

- Status: Proposed
- Date: 2026-11-10
- Deciders: Serban
- Related requirements: all (quality gate)

## Context
Every step lands through a pull request into `main`. Reviews should focus on intent, not on whether the code compiles
or is formatted.

## Options considered
### GitHub Actions: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`, `docker build`
- Free for public repos, lives next to the code. Con: Rust builds are slow without caching; we cache `~/.cargo` and `target`.
### No CI, run checks locally
- Nothing to maintain. Con: relies on discipline; reviewers cannot see that checks passed.
### Railway build only
- Catches Docker breakage. Con: no tests, and only after merge.

## Decision
**GitHub Actions** on every PR and on push to `main`.

## Consequences
- A red check blocks merging by convention (branch protection can enforce it later).
- The Docker build in CI catches Dockerfile breakage before Railway does.
