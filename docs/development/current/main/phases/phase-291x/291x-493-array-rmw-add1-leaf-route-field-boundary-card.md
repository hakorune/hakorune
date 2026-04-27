---
Status: Landed
Date: 2026-04-27
Scope: Make ArrayRmwAdd1LeafSeedRoute field layout owner-private
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-492-array-string-store-route-field-boundary-card.md
  - src/mir/array_rmw_add1_leaf_seed_plan.rs
  - src/mir/exact_seed_backend_route.rs
  - src/mir/mod.rs
  - src/runner/mir_json_emit/root.rs
  - src/runner/mir_json_emit/tests/exact_seed_backend_route.rs
---

# 291x-493: ArrayRmwAdd1Leaf Route Field Boundary

## Goal

Keep the array RMW add1 leaf seed route record owned by
`array_rmw_add1_leaf_seed_plan`.

The route remains public as function metadata, but its field layout and
route-local proof enum should not be the consumer contract.

## Inventory

Current external consumers:

- `src/runner/mir_json_emit/root.rs`
  - reads route fields directly for MIR JSON emission
- `src/runner/mir_json_emit/tests/exact_seed_backend_route.rs`
  - manually constructs the seed route
- `src/mir/exact_seed_backend_route.rs`
  - reads the proof field when selecting the exact backend route
  - test code manually constructs the seed route
- `src/mir/mod.rs`
  - root-exports both route and proof enum

Owner-local consumers:

- `src/mir/array_rmw_add1_leaf_seed_plan.rs`
  - owns detection, route field derivation, proof vocabulary, and tests

## Cleaner Boundary

```text
array_rmw_add1_leaf_seed_plan
  owns route fields and proof enum
  exposes stable read accessors
  exposes cfg(test) fixture builder for cross-module tests

JSON emitter / exact-seed selector
  consume accessors only
```

## Boundaries

- BoxShape-only.
- Do not change route detection.
- Do not change JSON field names or values.
- Do not change exact-seed route selection or priority.
- Do not change `.inc` behavior, helper symbols, or lowering.

## Acceptance

- `ArrayRmwAdd1LeafSeedRoute` fields are private.
- `ArrayRmwAdd1LeafSeedProof` is owner-private and no longer root-exported.
- JSON emitter and exact-seed selector read through accessors.
- Cross-module test construction uses an owner `cfg(test)` fixture.
- `cargo check -q` passes.
- Array RMW add1 leaf and exact-seed focused tests pass.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.

## Result

Landed as a BoxShape-only cleanup.

- `ArrayRmwAdd1LeafSeedRoute` now keeps its fields owner-private.
- `ArrayRmwAdd1LeafSeedProof` is owner-private and no longer root-exported.
- JSON emission and exact-seed selection consume route accessors only.
- Cross-module tests construct the route through the owner `cfg(test)` fixture.
- No `.inc`, helper symbol, lowering, or route priority behavior changed.

Verification:

```bash
cargo check -q
cargo test -q array_rmw_add1_leaf_seed
cargo test -q exact_seed_backend_route
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

Note: the quick gate still reports the pre-existing chip8 release artifact sync
warning, then finishes with `[dev-gate] profile=quick ok`.
