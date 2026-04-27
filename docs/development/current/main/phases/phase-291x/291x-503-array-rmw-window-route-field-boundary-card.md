---
Status: Landed
Date: 2026-04-27
Scope: Make ArrayRmwWindowRoute field layout owner-private
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-502-string-direct-set-window-route-field-boundary-card.md
  - src/mir/array_rmw_window_plan.rs
  - src/mir/array_rmw_add1_leaf_seed_plan.rs
  - src/mir/array_getset_micro_seed_plan.rs
  - src/runner/mir_json_emit/root.rs
  - src/runner/mir_json_emit/tests/array_routes.rs
---

# 291x-503: Array RMW Window Route Field Boundary

## Goal

Keep the `array.get(i) -> +1 -> array.set(i, ...)` window route field layout
owned by `array_rmw_window_plan`.

The route record remains public as function metadata. Its proof enum remains
public in this card because downstream exact seed routes still carry the inner
RMW proof as explicit contract vocabulary.

## Inventory

Current external consumers:

- `src/mir/array_rmw_add1_leaf_seed_plan.rs`
  - selects and copies inner RMW route fields directly
  - tests manually construct inner RMW routes
- `src/mir/array_getset_micro_seed_plan.rs`
  - selects and copies inner RMW route fields directly
  - tests manually construct inner RMW routes
- `src/runner/mir_json_emit/root.rs`
  - reads route fields directly for JSON metadata
- `src/runner/mir_json_emit/tests/array_routes.rs`
  - manually constructs the route for JSON emission tests

Owner-local consumers:

- `src/mir/array_rmw_window_plan.rs`
  - owns detection, route field derivation, proof vocabulary, and focused tests

## Cleaner Boundary

```text
array_rmw_window_plan
  owns route fields
  exposes stable read accessors
  exposes cfg(test) fixture builders for cross-module tests

seed route planners / JSON emitter
  consume accessors/fixtures only
```

## Boundaries

- BoxShape-only.
- Do not change RMW window detection.
- Do not change downstream seed route selection.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.
- Do not make `ArrayRmwWindowProof` private in this card.

## Acceptance

- `ArrayRmwWindowRoute` fields are private.
- Downstream seed planners read the inner route through accessors.
- Runner JSON emission reads through accessors.
- Cross-module tests use owner `cfg(test)` fixtures instead of constructing
  route fields directly.
- `cargo check -q` passes.
- RMW window and downstream seed focused tests pass.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.

## Result

- `ArrayRmwWindowRoute` fields are owner-private.
- `ArrayRmwWindowProof` intentionally remains public because downstream exact
  seed routes still carry the inner RMW proof vocabulary.
- Added stable read accessors for seed planners and JSON emission.
- Added owner `cfg(test)` fixtures for downstream seed and runner JSON tests.
- Updated downstream seed planners and JSON emission to consume accessors and
  fixtures instead of route field layout.
- Preserved RMW route detection, downstream seed selection, JSON field
  names/values, `.inc` behavior, helper symbols, and lowering.

## Verification

```bash
cargo check -q
cargo test -q array_rmw
cargo test -q array_getset
cargo test -q array_rmw_add1_leaf
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

`tools/checks/dev_gate.sh quick` passed with the pre-existing chip8 release
artifact sync warning still reported before the final profile success line.
