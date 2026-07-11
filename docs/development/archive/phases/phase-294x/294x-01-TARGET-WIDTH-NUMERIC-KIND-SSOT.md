---
Status: Complete
Date: 2026-05-12
Scope: code-side target pointer-width and target-resolved numeric kind SSOT.
Related:
  - src/mir/numeric_substrate.rs
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - docs/reference/language/types.md
  - docs/reference/runtime/substrate-capabilities.md
---

# 294x-01 Target Width Numeric Kind SSOT

## Goal

Give `usize` / `isize` a code-side target-width owner before any runtime,
backend, or hako_alloc migration row claims exact pointer-sized semantics.

## Changes

- Added `NumericTarget` as the pointer-width owner.
- Added `NumericResolvedWidth` for fixed target-resolved widths.
- Added `NumericKind` as signedness plus resolved width.
- Added `classify_numeric_kind_for_target(...)`.
- Kept `classify_numeric_type_name(...)` for source spelling metadata.
- Kept runtime behavior unchanged: numeric substrate names still map to the
  current integer lane until later 294x rows add exact semantics.

## Stop Line

This row does not add:

- exact `usize` runtime values;
- unsigned comparisons;
- logical right shift;
- overflow or range verification;
- typed-object `usize` storage;
- backend lowering to native pointer-sized integer classes;
- hako_alloc field migration.

## Proof

```bash
cargo test -q numeric_substrate --lib
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
