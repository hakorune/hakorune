---
Status: Complete
Date: 2026-05-12
Scope: exact numeric PHI/Select control-merge policy.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/README.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x-09 PHI Select Numeric Unification Policy

## Purpose

Define how exact numeric type facts may survive PHI/Select control merges.

This row keeps the policy conservative: exact numeric facts are preserved only
when every incoming exact fact is the same source/resolved type. Mixed
exact/dynamic values and mismatched exact types fail fast instead of silently
dropping `usize` semantics back to the dynamic `Integer(i64)` lane.

## Landed

- Added `src/mir/exact_numeric_unification.rs` as a separate policy owner so
  `numeric_substrate.rs` does not keep growing.
- Added:
  - `ExactNumericMergeSite::{Phi, Select}`;
  - `ExactNumericUnificationError::{MixedExactAndDynamic, TypeMismatch}`;
  - `unify_exact_numeric_control_merge(...)`.
- Unit tests now fix:
  - all-dynamic merge returns no exact type;
  - same exact `usize` merge preserves the type;
  - exact/dynamic Select merge rejects;
  - exact type mismatch rejects.

## Contract

```text
all incoming are dynamic/non-exact:
  Ok(None)

all incoming exact types are identical:
  Ok(Some(exact type))

exact + dynamic/non-exact mix:
  MixedExactAndDynamic

different exact source/resolved types:
  TypeMismatch
```

The policy intentionally requires exact type equality, including source
spelling. A future row may relax aliases such as target-32 `usize` and `u32`,
but that must be an explicit decision.

## Non-Goals

- wiring PHI/Select facts into MIR builder/lowerer;
- changing existing PHI/Select runtime behavior;
- exact numeric VM value routes;
- backend lowering;
- hako_alloc migration.

## Verification

- `cargo test -q exact_numeric_unification --lib`
- `cargo check --bin hakorune`

## Next

1. attach exact numeric route facts for params/locals/control merges;
2. add VM exact `usize` value/ops rows;
3. migrate hako_alloc fields only after live semantics are available.
