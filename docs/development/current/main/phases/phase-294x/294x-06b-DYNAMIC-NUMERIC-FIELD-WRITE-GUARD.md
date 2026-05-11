---
Status: Complete
Date: 2026-05-12
Scope: verifier guard for dynamic exact numeric field writes that need runtime
  range checks.
Related:
  - src/mir/verification/numeric_substrate.rs
  - src/mir/verification_types.rs
  - src/mir/numeric_substrate.rs
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x-06b Dynamic Numeric Field Write Guard

## Goal

Prevent exact numeric fields from silently accepting dynamic `Integer(i64)`
values when the declared field type needs a range check that is not represented
in MIR/runtime yet.

This row keeps `i64` compatible with the current runtime lane while blocking
`usize`, unsigned types, and narrower signed types until a later row adds an
explicit runtime-check contract.

## Changes

- Added a numeric-substrate helper that detects whether an exact numeric type's
  range fails to cover the full dynamic `Integer(i64)` range.
- The MIR verifier now reports
  `VerificationError::ExactNumericDynamicCheckRequired` when a `FieldSet`
  writes an unresolved dynamic value into such a field.
- Stable diagnostic display tag:
  - `[mir/verify:numeric_dynamic_check_required]`.
- Producer labels are tracked through simple `Copy` chains for diagnostics:
  - `param`;
  - `binop`;
  - `call`;
  - other MIR producer categories as stable strings.

## Accepted Surface

For exact numeric fields:

- static integer constants still use the 294x-06 range check;
- dynamic values are accepted only when the exact type range covers every
  possible `Integer(i64)` value, such as `i64` on the current 64-bit target;
- dynamic values for `usize`, unsigned types, and narrower signed types
  fail-fast until runtime check lowering exists.

## Stop Line

This row does not add:

- VM exact `usize` values;
- runtime range-check instructions;
- checked arithmetic;
- unsigned comparison or logical shift;
- typed-object exact numeric storage;
- backend lowering;
- hako_alloc field migration.

## Next Rows

The next safe order is:

1. runtime-check contract metadata for dynamic exact numeric writes;
2. VM construction/range-check v0 for `usize`;
3. checked arithmetic policy and implementation;
4. unsigned compare and logical shift;
5. PHI/Select exact numeric unification.

## Proof

```bash
cargo test -q numeric_substrate --lib
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
