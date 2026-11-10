---
name: writing-adr
description: Use when making or changing an architecture decision for the calculator API. Writes one numbered ADR per decision in docs/adr/ with context, real options, trade-offs and consequences.
---

# Writing an ADR

One decision per file: `docs/adr/NNNN-short-title.md` (for example, `0002-exact-decimal-arithmetic.md`).
ADRs are immutable once accepted: to change a decision, write a new ADR that supersedes the old one.

## Template

```markdown
# NNNN. Title in the form of a decision

- Status: Proposed | Accepted | Superseded by NNNN
- Date: YYYY-MM-DD
- Deciders: <names>
- Related requirements: FR-x, NFR-y

## Context
What forces are at play: the requirement, the constraints, what we know and don't know.

## Options considered
### Option A
Pros / cons.
### Option B
Pros / cons.
(At least two real options. "Do nothing" counts if it is realistic.)

## Decision
What we chose, in one or two sentences.

## Consequences
What becomes easier, what becomes harder, what we must now do (tests, docs, ops),
and what would make us revisit this decision.
```

## Rules

- Trace every ADR to the PRD requirement(s) that force it. If none do, question whether the decision is needed.
- Options must be genuine alternatives with honest downsides for the chosen one.
- Keep each ADR short: if it needs more than a page, it is probably two decisions.
- The owner accepts the ADR. Until then its status stays `Proposed`.
- Keep an index in `docs/adr/README.md`.
