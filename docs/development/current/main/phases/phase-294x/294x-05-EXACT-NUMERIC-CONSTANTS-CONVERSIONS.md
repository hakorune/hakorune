---
Status: Complete
Date: 2026-05-12
Scope: exact numeric constant/range and dynamic integer conversion model.
Related:
  - src/mir/numeric_substrate.rs
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x-05 Exact Numeric Constants Conversions

## Goal

Give exact numeric metadata a range-checked constant/conversion vocabulary
before verifier, runtime, or backend rows consume it.

## Changes

- Added bit-width range computation for `NumericKind`.
- Added `ExactNumericConstValue` as exact numeric constant metadata.
- Added `ExactNumericConversionError` with:
  - `NegativeToUnsigned`;
  - `OutOfRange`.
- Added conversion helpers for:
  - exact constants from `i128`;
  - dynamic `Integer(i64)` values into exact numeric metadata;
  - declared type names into optional exact numeric conversions.
- Kept non-numeric declared names as `Ok(None)` so legacy Box type names do not
  become conversion failures.

## Stop Line

This row does not add:

- suffixed numeric literal syntax;
- verifier assignment checks;
- runtime exact `usize` values;
- arithmetic overflow policy;
- unsigned compare or logical shift;
- backend lowering;
- hako_alloc field migration.

The conversion model is metadata-only. Later rows decide when failures become
compile-time verifier errors or runtime fail-fast diagnostics.

## Proof

```bash
cargo check --bin hakorune
cargo test -q numeric_substrate --lib
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
