# 0002. Use rust_decimal for all arithmetic

- Status: Proposed
- Date: 2026-11-10
- Deciders: Serban
- Related requirements: FR-2, FR-3, FR-4, FR-5

## Context
The core promise of the product is exact decimal results: `0.1 + 0.2` must return `0.3` (FR-4). Binary floating point
cannot represent most decimal fractions. Results must round half-to-even at 28 significant digits, and overflow must be an error.

## Options considered
### f64
- Fast, built in. Con: `0.1 + 0.2 = 0.30000000000000004`; violates FR-4. Rejected.
### rust_decimal (96-bit decimal, 28 significant digits)
- Exact base-10 arithmetic; `checked_add/sub/mul/div/rem` return `None` on overflow or division by zero.
- Precision and rounding match FR-4 exactly; `sqrt` and integer `powi` available behind the `maths` feature.
- `serde-str` style string (de)serialisation fits our "numbers as strings" API.
- Con: bounded range (about ±7.9 × 10^28); very large results become `OVERFLOW`.
### bigdecimal (arbitrary precision)
- No overflow. Con: unbounded precision means `1 ÷ 3` needs an explicit precision anyway, and a hostile
  `power` request can exhaust CPU and memory (NFR-5). More rules for no requirement that asks for it.

## Decision
Use **rust_decimal**, only through its `checked_*` operations. `f64` must never appear in the calculation path.

## Consequences
- Every operation returns `Result<Decimal, CalcError>`; overflow maps to `OVERFLOW`, zero divisors to `DIVISION_BY_ZERO`.
- Inputs are parsed with a strict parser that rejects scientific notation and values that would lose precision.
- `sqrt` results are rounded per FR-4; tests must pin a few known values.
- If a future requirement needs bigger numbers, revisit with bigdecimal plus explicit limits.
