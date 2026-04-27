---
Status: Landed
Date: 2026-04-27
Scope: Make ExactSeedBackendRoute field layout owner-private and update JSON consumers
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-489-map-lookup-fusion-root-export-prune-card.md
  - src/mir/exact_seed_backend_route.rs
  - src/mir/mod.rs
  - src/runner/mir_json_emit/root.rs
  - src/runner/mir_json_emit/tests/exact_seed_backend_route.rs
---

# 291x-490: ExactSeedBackendRoute Field Boundary

## Goal

Keep exact-seed backend route construction and route-local vocabulary inside
`exact_seed_backend_route`.

`ExactSeedBackendRoute` is function metadata, so the route record itself remains
public through `MirFunction` metadata. Its field layout and route-local `Kind`
should not be the consumer contract.

## Inventory

Current external consumers:

- `src/runner/mir_json_emit/root.rs`
  - reads `tag`, `source_route`, `proof`, and `selected_value` directly
- `src/runner/mir_json_emit/tests/exact_seed_backend_route.rs`
  - manually constructs exact backend route records with public fields
  - imports `ExactSeedBackendRouteKind` through root `crate::mir`
- `src/mir/mod.rs`
  - re-exports both `ExactSeedBackendRoute` and `ExactSeedBackendRouteKind`

Owner-local consumers:

- `src/mir/exact_seed_backend_route.rs`
  - derives routes from already-proven seed metadata
  - owns route tags, source-route field names, proof strings, and selected value

## Cleaner Boundary

```text
exact_seed_backend_route
  owns route fields and route kind
  exposes stable read accessors for JSON
  exposes cfg(test) fixture builders for JSON tests

runner JSON emitter/tests
  consume accessors/fixtures only
```

## Boundaries

- BoxShape-only.
- Do not change exact route selection or priority.
- Do not change JSON field names or values.
- Do not change seed route metadata, `.inc` readers, helper symbols, or lowering.
- Do not add new exact-seed route variants.

## Acceptance

- `ExactSeedBackendRoute` fields are private.
- `ExactSeedBackendRouteKind` is owner-private and no longer root-exported.
- JSON emitter reads through route accessors only.
- Runner JSON exact-seed tests use owner test fixtures for exact route records.
- `cargo check -q` passes.
- Exact seed backend route JSON and planner tests pass.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.

## Result

Landed.

- Made `ExactSeedBackendRoute` fields owner-private.
- Made `ExactSeedBackendRouteKind` owner-private and removed its root
  `crate::mir` re-export.
- Added stable read accessors for JSON emission:
  - `tag()`
  - `source_route()`
  - `proof()`
  - `selected_value()`
- Added owner-local `cfg(test)` fixture builders for exact backend route JSON
  tests.
- Updated the JSON emitter and runner JSON exact-seed tests to consume
  accessors/fixtures rather than route field layout.
- Preserved route selection, priority, JSON field names/values, seed metadata,
  `.inc` behavior, helper symbols, and lowering.

Verification:

```bash
cargo check -q
cargo test -q build_mir_json_root_emits_exact_seed_backend_route
cargo test -q exact_seed_backend_route
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

Note: quick gate still reports the existing chip8 release artifact sync
warning, then completes with `[dev-gate] profile=quick ok`.
