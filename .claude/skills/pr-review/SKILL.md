---
name: pr-review
description: Use when asked to review a branch, diff or pull request in this repo (/pr-review). Checks the change against the PRD, ADRs and plan first, then correctness, tests and Rust quality, and reports findings ranked by severity.
---

# PR review

Review like the person who will be paged when this breaks. Spec-driven project: the docs are the contract.

## 0. Gather

- The diff (`git diff main...HEAD`, or the PR's changes) **and** the code it touches, not only the changed lines.
- The task(s) in `docs/plan/` this change claims to implement, and the PRD requirements and ADRs they reference.

## 1. Spec conformance (most important)

- Does the change do what the referenced FR/NFR says: no less, no more?
- Does it contradict an accepted ADR? If it intentionally diverges, is there a new ADR or doc update in the same PR?
- Is anything built that no requirement asks for (scope creep)?

## 2. Correctness

- Decimal handling: no `f64` anywhere in the calculation path; overflow and division by zero handled.
- Errors follow `api-conventions` (shape, status codes, no leaked internals).
- No `unwrap()`/`expect()`/panics reachable from a request.

## 3. Tests

- Every new FR or edge case has a test, named after the requirement.
- Tests assert behaviour (status + body), not implementation details.
- Run: `cargo fmt --check && cargo clippy -- -D warnings && cargo test`. Report the actual output.

## 4. Simplicity and operations

- Could this be smaller (ponytail)? Unneeded dependencies, abstractions, config?
- Docker image still builds; `PORT` still honoured; no secrets committed.

## Output

A ranked list. For each finding: **severity** (blocker / should-fix / nit), **file:line**, what is wrong,
a concrete failure scenario, and a suggested fix. End with a verdict: `APPROVE`, `APPROVE WITH NITS` or `REQUEST CHANGES`,
plus anything you could not check.

To also get a second opinion, run ponytail's `/ponytail-review` and superpowers `requesting-code-review`.
