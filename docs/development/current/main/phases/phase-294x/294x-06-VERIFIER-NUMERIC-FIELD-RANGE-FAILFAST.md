---
Status: Complete
Date: 2026-05-12
Scope: verifier fail-fast for statically known exact numeric field writes.
Related:
  - src/mir/verification/numeric_substrate.rs
  - src/mir/verification.rs
  - src/mir/verification_types.rs
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x-06 Verifier Numeric Field Range Failfast

## Goal

Start consuming exact numeric metadata from the MIR verifier without changing
VM values, backend lowering, or the legacy `Integer(i64)` runtime lane.

## Changes

- Added `src/mir/verification/numeric_substrate.rs` as the numeric verifier
  owner.
- The verifier now builds a module-local map from
  `ModuleMetadata::user_box_field_decls`.
- For `FieldSet` instructions, the verifier resolves:
  - same-function `NewBox` / `Copy` chains for the object box;
  - same-function integer `Const` / `Copy` chains for the assigned value.
- Exact numeric declared fields now reject statically known out-of-range
  integer writes with `VerificationError::ExactNumericRangeViolation`.
- Stable diagnostic display tag:
  - `[mir/verify:numeric_range]`.

## Accepted Surface

This row checks only field writes where both sides are statically visible in
the current MIR function:

- base object known from `NewBox` or `me`/Box-typed params;
- field declared type known from module metadata;
- assigned value known as `ConstValue::Integer` or a simple `Copy` chain.

This is enough to reject `usize` fields initialized with `-1` and small exact
numeric fields such as `u8` initialized with `256`.

## Stop Line

This row does not add:

- exact numeric `MirType`;
- param declared-type verification;
- local variable type annotations;
- dynamic value range proof;
- runtime exact `usize` values;
- arithmetic overflow checks;
- unsigned comparison or logical shift;
- backend lowering;
- hako_alloc field migration.

## Proof

```bash
cargo check --bin hakorune
cargo test -q numeric_substrate --lib
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
