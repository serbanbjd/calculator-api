# Calculator API: agent guide

A stateless HTTP calculator API in **Rust** (axum), shipped as a **Docker** image and deployed on **Railway**.
Built with spec-driven development: documentation comes first, code follows it.

## Workflow (spec-driven)

Each step produces a reviewed document before the next one starts. Never skip ahead.

1. **PRD** in `docs/prd/`: what the calculator does, and what it does not do. Skill: `writing-prd`.
2. **ADR** in `docs/adr/`: architecture decisions, one per file, numbered. Skill: `writing-adr`.
3. **Plan + tasks** in `docs/plan/`: ordered tasks traced back to PRD requirements, with a UML diagram (Mermaid).
   Use superpowers `writing-plans`.
4. **Code**: implement task by task with superpowers `test-driven-development` and `executing-plans`.
   Follow `api-conventions`. Keep code minimal (ponytail), but never cut validation, error handling or tests.
5. **Review** every change with `pr-review` before it is merged.

If code needs to diverge from a doc, update the doc in the same commit and say why.

## Libraries (Claude Code plugins)

Declared in `.claude/settings.json`; Claude Code offers to install them when the repo is trusted.

- **superpowers** (`obra/superpowers`): brainstorming, writing and executing plans, TDD, systematic debugging, code review.
- **ponytail** (`DietrichGebert/ponytail`): minimal code without skipping safety; `/ponytail-review`, `/ponytail-audit`.

Manual install:
```
/plugin marketplace add obra/superpowers-marketplace
/plugin install superpowers@superpowers-marketplace
/plugin marketplace add DietrichGebert/ponytail
/plugin install ponytail@ponytail
```

## Project skills (`.claude/skills/`)

| Skill | Use it when |
|---|---|
| `writing-prd` | defining or changing product requirements |
| `writing-adr` | making an architecture decision |
| `api-conventions` | writing or reviewing any endpoint |
| `pr-review` | reviewing a branch or pull request (`/pr-review`) |

## Fixed decisions

- Numbers are exact decimals (`rust_decimal`), never `f64`: `0.1 + 0.2` must equal `0.3`.
- Config comes from the environment only (`PORT`, default 8080).

## Commands

```
cargo fmt --check && cargo clippy -- -D warnings && cargo test
docker build -t calculator-api . && docker run -p 8080:8080 calculator-api
```

## Commits

Small, one logical step each, with a message that says why. Docs and the code that implements them stay in sync.
