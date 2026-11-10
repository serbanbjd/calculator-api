# Calculator API

A stateless HTTP calculator with **exact decimal arithmetic** (`0.1 + 0.2 = 0.3`), written in Rust (axum),
shipped as a Docker image and deployable on Railway. Built with AI using spec-driven development:
PRD → ADRs → plan → code, each step reviewed in its own pull request.

| Document | What it holds |
|---|---|
| [`docs/prd/PRD.md`](docs/prd/PRD.md) | requirements (FR/NFR), non-goals, acceptance criteria |
| [`docs/adr/`](docs/adr/README.md) | seven architecture decisions with alternatives |
| [`docs/plan/PLAN.md`](docs/plan/PLAN.md) | tasks T1–T11, UML class and sequence diagrams, test strategy |
| [`CLAUDE.md`](CLAUDE.md), [`.claude/`](.claude) | the agent setup: workflow, plugins (superpowers, ponytail), project skills |

## Run it

```sh
cargo run                                   # http://localhost:8080
# or
docker build -t calculator-api . && docker run -p 8080:8080 calculator-api
```

Then open **http://localhost:8080/docs** for Swagger UI.

```sh
curl -X POST localhost:8080/v1/calculate -H 'content-type: application/json' \
     -d '{"op":"add","a":"0.1","b":"0.2"}'
# {"result":"0.3"}

curl -X POST localhost:8080/v1/calculate -H 'content-type: application/json' \
     -d '{"op":"divide","a":"1","b":"0"}'
# 422 {"error":{"code":"DIVISION_BY_ZERO","message":"Cannot divide by zero"}}
```

Operations: `add`, `subtract`, `multiply`, `divide`, `modulo`, `power` (integer exponent), `percent` (a % of b), `sqrt` (only `a`).
Numbers are JSON strings. Routes: `POST /v1/calculate`, `GET /health`, `GET /openapi.json`, `GET /docs`.

## Develop

```sh
cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test
```

Deploy: Railway builds the `Dockerfile` and health-checks `/health` (see [`railway.json`](railway.json)).
`PORT` is the only configuration.

## How this was built

1. **Agent setup**: `.claude/settings.json` pulls in the [superpowers](https://github.com/obra/superpowers) and
   [ponytail](https://github.com/DietrichGebert/ponytail) plugins; `.claude/skills/` adds `writing-prd`, `writing-adr`,
   `api-conventions` and `pr-review`.
2. **PRD** written from an interview, reviewed in PR #1.
3. **ADRs** reviewed and accepted in PR #2.
4. **Plan** with UML diagrams, reviewed in PR #3.
5. **Code** generated task by task, test-first, in PR #4.

## Reflections

<!-- Written by Serban. -->

### How I defined the boundaries of the system

_TODO_

### How I gathered the PRD

_TODO_

### Plans, tasks and PRD: when can they be removed, and what follows

_TODO_

### How I review AI-generated documentation (ADR, PRD, plans)

_TODO_

### When team members should review

_TODO_

### Test types: what I added and why, legacy vs green-field

_TODO_
