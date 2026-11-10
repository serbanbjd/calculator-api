---
name: writing-prd
description: Use when defining or changing product requirements for the calculator API. Interviews the owner first, then writes docs/prd/PRD.md with numbered, testable requirements and explicit non-goals.
---

# Writing a PRD

A PRD says **what** the system does and **why**, never **how**. The owner decides scope; you ask and write.

## 1. Interview before writing

Ask one question at a time (superpowers `brainstorming` style). Do not write the PRD until these are answered:

- Who calls this API, and from where (browser, backend, CLI)?
- Which operations are in scope? Which are explicitly out?
- Input shape: separate operands, or an expression string?
- Number rules: precision, range, rounding, division by zero, overflow.
- Is it stateless, or does it remember anything (history, memory)?
- Non-functional needs: latency, payload limits, auth, rate limits, availability.
- How will we know it is done (acceptance criteria)?

If the owner does not know, propose a default and mark it `ASSUMPTION` in the PRD.

## 2. Structure of `docs/prd/PRD.md`

1. **Problem and goal**: two or three sentences.
2. **Users and use cases**
3. **System boundary**: what is inside, what is outside, and what it depends on.
4. **Functional requirements**: `FR-1`, `FR-2`, ... each one testable, with an example request and response.
5. **Non-functional requirements**: `NFR-1`, ... each one with a number (for example, p95 < 50 ms, body ≤ 16 KB).
6. **Non-goals**: what we deliberately do not build.
7. **Assumptions and open questions**
8. **Acceptance criteria**: one or more per FR.

## 3. Rules

- Every requirement has an ID; plans, tasks and tests reference those IDs.
- No technology choices in the PRD (they belong in an ADR).
- Prefer fewer, sharper requirements over long lists.
- Show the draft to the owner and record their changes before committing.
