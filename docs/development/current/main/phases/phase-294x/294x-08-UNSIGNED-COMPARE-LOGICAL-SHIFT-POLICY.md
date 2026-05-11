---
Status: Complete
Date: 2026-05-12
Scope: exact numeric compare and logical right-shift policy.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/README.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x-08 Unsigned Compare Logical Shift Policy

## Purpose

Define the exact numeric comparison and logical right-shift policy before VM or
backend lowering routes start executing `usize` operations.

This row prevents future `usize` lowering from borrowing signed i64 comparison
or arithmetic-right-shift semantics by accident.

## Landed

- Added exact numeric compare vocabulary to `src/mir/numeric_substrate.rs`:
  - `ExactNumericCompareOp::{Eq, Ne, Lt, Le, Gt, Ge}`;
  - `ExactNumericCompareError::TypeMismatch`;
  - `exact_numeric_compare(...)`.
- Added logical right-shift policy:
  - `ExactNumericShiftError::{SignedLogicalShift, ShiftCountOutOfRange}`;
  - `exact_numeric_logical_shr(...)`.
- Unit tests now fix:
  - high-bit `usize` comparison as unsigned value order;
  - compare type mismatch rejection;
  - logical right shift of the high unsigned bit;
  - signed logical shift rejection;
  - shift count at width rejection.

## Contract

Exact numeric compare:

```text
same exact type:
  compare exact values

type mismatch:
  fail fast with TypeMismatch
```

Exact numeric logical right shift:

```text
unsigned exact type + shift < width:
  return value >> shift

signed exact type:
  fail fast with SignedLogicalShift

shift >= width:
  fail fast with ShiftCountOutOfRange
```

## Non-Goals

- VM execution of exact compare or shift;
- backend lowering of unsigned compare or logical shift;
- signed logical-shift syntax;
- arithmetic right-shift exact policy;
- PHI/Select numeric unification;
- hako_alloc field migration.

## Verification

- `cargo test -q numeric_substrate --lib`
- `cargo check --bin hakorune`

## Next

1. add PHI/Select exact numeric unification;
2. add VM exact `usize` value/ops rows that consume the policy helpers;
3. lower backend support only after VM semantics are fixed.
