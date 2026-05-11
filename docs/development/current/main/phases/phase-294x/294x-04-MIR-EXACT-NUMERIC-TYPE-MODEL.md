---
Status: Complete
Date: 2026-05-12
Scope: MIR-owned exact numeric type side model.
Related:
  - src/mir/numeric_substrate.rs
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x-04 MIR Exact Numeric Type Model

## Goal

Represent exact numeric signedness and resolved width in a MIR-owned model that
is distinct from the legacy `MirType::Integer` lane.

## Changes

- Added `ExactNumericMirType` as source spelling plus target-resolved
  `NumericKind`.
- Added `ExactNumericMirSignature` as param/return exact numeric metadata.
- Added helpers to derive exact numeric MIR metadata from declared type names.
- Kept non-numeric type names as `None`, so legacy box names do not masquerade
  as exact integers.

## Stop Line

This row does not add:

- `MirType` variant changes;
- function signature lowering to exact numeric types;
- route facts for numeric params/returns;
- verifier range/overflow checks;
- VM/runtime exact `usize` values;
- backend exact unsigned lowering;
- hako_alloc field migration.

The model is intentionally side-car metadata. Later rows decide where it is
attached to MIR facts and lowerers.

## Proof

```bash
cargo check --bin hakorune
cargo test -q numeric_substrate --lib
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
