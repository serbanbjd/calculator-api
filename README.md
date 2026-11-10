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

**Live:** https://calculator-api-production-1ece.up.railway.app/docs (Railway, deploys from `main`).

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
5. **Code** generated task by task, test-first, in PR #4, then deployed to Railway.

## Reflections

_Written by Serban, with AI help on wording._

### How I defined the boundaries of the system

Given the scope of the task, I kept only the minimum needed to satisfy the requirements, to avoid building functionality nobody asked for. Concretely: one operation per request (no expression parsing), a stateless service (no history or memory), and no auth or rate limiting. Callers can be anyone (a UI, curl, an AI agent), so the boundary is the HTTP contract itself. The only things I added beyond the brief were a Docker image and a Railway deployment, because I consider being able to build and deploy from day one a good foundation for any project, and Swagger UI at `/docs` so anyone can try the API.

### How I gathered the PRD

The functionality was straightforward, so I gathered the PRD through a short interview with the AI: it asked one question at a time, proposed a default for each, and marked anything I hadn't decided as an ASSUMPTION. I steered it in the critical directions myself. I chose the feature list (eight operations, one per call, no expressions), exact decimal arithmetic instead of floating point (`0.1 + 0.2` must be `0.3`), how edge cases behave (fractional powers rejected, no scientific notation), and the technical direction of Rust and Docker. I then reviewed the draft in its own pull request. There I confirmed or removed the assumptions, for example dropping a latency target that a service this simple doesn't need, before any architecture work started. On a real project, this interview is the moment to bring in the stakeholders: they would answer the feature questions and confirm the assumptions, with the AI structuring the conversation and turning it into testable requirements.

### Plans, tasks and PRD: when can they be removed, and what follows

Plans and tasks are working documents: once the tasks are delivered and merged, they have served their purpose and can be removed. The PRD is different. In a spec-driven project, I'd keep it as the living source of truth for what the product does: every change starts by updating the PRD, and the plan and code follow from it. That way the PRD itself becomes the well-structured documentation we need, backed by tests named after its acceptance criteria and by the OpenAPI contract. ADRs work differently. Over a product's lifetime they start to overlap (ADR-0001 says one thing, ADR-2450 changes part of it), so they serve as history of why things changed. The current decisions should be folded into the living documentation rather than read from hundreds of records. In teams where the PRD lives in a ticket, it is a snapshot that gets closed when the feature ships. There, the important information should be moved into the living documentation before the ticket is closed, or it gets lost.

### How I review AI-generated documentation (ADR, PRD, plans)

When reviewing AI-generated documentation, I focus on two places. First, the feature list: I check that it is exactly what we asked for. The AI tends to add things that sound reasonable but nobody requested, like the latency target it put in the PRD, which I removed. Small deviations can be acceptable, but each one should be a conscious decision, not something that slipped in. Second, the technical proposal in the ADRs and the plan: I check that it respects our existing standards and doesn't reinvent them per service. For example, authentication should be handled universally across the platform, not implemented separately inside each service. Those standards should normally already be written down for the agent, in `CLAUDE.md`, rules or skills. But review is also where I find what we forgot to specify. Every such gap becomes a new rule or skill, so the next document starts out right instead of being corrected again.

### When team members should review

Review should happen at each step, by the people who own that step. Product reviews the PRD, to make sure the features are the right ones. Architects review the ADRs, to make sure the technical direction fits our standards and the implementation will be done correctly. Developers review the code. Pull requests should be small, because that is what makes a real review possible: a 50-file pull request will probably not get much scrutiny. This raises the harder question of whether everything should always be reviewed, since every review costs speed. Which changes, product proposals or ADRs need a review, and which can go straight through? I don't think anyone has found the perfect balance yet, and it depends on the project. My starting point would be to review where a mistake is expensive to undo: the PRD and ADRs always, since a wrong decision there multiplies through everything built on it, and code in proportion to its risk. In this project the code pull request (#4) was bigger than ideal. Each task was a separate commit, but splitting it per task would have made review easier.

### Test types: what I added and why, legacy vs green-field

**What I added and why.** Unit tests on the domain logic make up most of the suite: every operation, rounding rule and error case, fast and isolated. API integration tests call the router in-process, with one test per acceptance criterion and error row in the PRD, and are named after the requirement they cover (`fr5_...`, `ac1_...`). Property-based tests throw random numbers, strings and raw bytes at the API and check it never crashes or returns a 5xx. A Docker smoke test in CI checks that the real image starts and answers. The tests earned their place: writing the API tests revealed that the JSON parser accepted a positional array (`["add","1","2"]`) as a valid request.

**Green-field vs legacy.** On a green-field project like this one, I'd build the classic pyramid from the start: mostly unit tests, then API tests tied to the requirements, then a few end-to-end checks. On a legacy project, unit tests may be missing or hard to write because the code wasn't designed for them. If the project already has a robust set of unit and API/integration tests, we keep and maintain them. If it doesn't, the first step is to set up a good end-to-end suite that captures how the system behaves today, before we start using agentic development on it. That suite is the safety net that tells us whether an AI-made change broke something.
