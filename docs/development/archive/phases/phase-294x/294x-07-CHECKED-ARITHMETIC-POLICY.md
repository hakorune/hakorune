---
Status: Complete
Date: 2026-05-12
Scope: exact numeric checked add/sub/mul policy in the MIR numeric substrate.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/README.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x-07 Checked Arithmetic Policy

## Purpose

Define the overflow policy for exact numeric arithmetic before VM/backend
lowering starts using `usize` as a live operation type.

This keeps `usize` honest for allocator work: plain exact arithmetic is checked
and fail-fast by policy. Wrapping arithmetic remains future explicit
vocabulary, not an implicit fallback to Rust/i64 wrapping behavior.

## Landed

- Added exact numeric arithmetic vocabulary to `src/mir/numeric_substrate.rs`:
  - `ExactNumericArithmeticOp::{Add, Sub, Mul}`;
  - `ExactNumericArithmeticError::{TypeMismatch, ResultOutOfRange}`;
  - `exact_numeric_checked_arithmetic(...)`.
- The policy checks:
  - both operands have the same exact numeric type;
  - the computed result fits the exact type's signedness/width range;
  - internal `i128` arithmetic overflow is reported as an out-of-range result
    with `result = None`.
- Unit tests now fix:
  - in-range `usize` add;
  - `usize` subtraction underflow;
  - narrow unsigned add overflow;
  - signed multiply overflow;
  - large `u64` multiply beyond `i128`;
  - mismatched exact numeric types.

## Contract

For exact numeric add/sub/mul:

```text
same exact type + result in type range:
  returns exact numeric const value

type mismatch:
  fail fast with TypeMismatch

result outside type range:
  fail fast with ResultOutOfRange

wrapping:
  not implicit
```

## Non-Goals

- VM execution of exact `usize` add/sub/mul;
- backend lowering of exact arithmetic;
- division/modulo/zero-check policy;
- unsigned compare or logical shift;
- syntax for wrapping/checked helper calls;
- hako_alloc field migration.

## Verification

- `cargo test -q numeric_substrate --lib`
- `cargo check --bin hakorune`

## Next

1. add unsigned compare and logical shift policy rows;
2. add exact VM value/operation rows that consume this policy;
3. lower backend support only after VM semantics are fixed.
