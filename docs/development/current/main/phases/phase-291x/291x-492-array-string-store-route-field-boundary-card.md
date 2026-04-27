---
Status: Landed
Date: 2026-04-27
Scope: Make ArrayStringStoreMicroSeedRoute field layout owner-private
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-491-concat-const-suffix-route-field-boundary-card.md
  - src/mir/array_string_store_micro_seed_plan.rs
  - src/mir/exact_seed_backend_route.rs
  - src/mir/mod.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-492: ArrayStringStore Route Field Boundary

## Goal

Keep the array/string-store micro seed route record owned by
`array_string_store_micro_seed_plan`.

The route remains public as function metadata, but its field layout and
route-local proof enum should not be the consumer contract.

## Inventory

Current external consumers:

- `src/runner/mir_json_emit/root.rs`
  - reads route fields directly for MIR JSON emission
- `src/mir/exact_seed_backend_route.rs`
  - reads the proof field when selecting the exact backend route
  - test code manually constructs the seed route
- `src/mir/mod.rs`
  - root-exports both route and proof enum

Owner-local consumers:

- `src/mir/array_string_store_micro_seed_plan.rs`
  - owns detection, route field derivation, proof vocabulary, and tests

## Cleaner Boundary

```text
array_string_store_micro_seed_plan
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

- `ArrayStringStoreMicroSeedRoute` fields are private.
- `ArrayStringStoreMicroSeedProof` is owner-private and no longer root-exported.
- JSON emitter and exact-seed selector read through accessors.
- Cross-module test construction uses an owner `cfg(test)` fixture.
- `cargo check -q` passes.
- Array string-store and exact-seed focused tests pass.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.

## Result

Landed.

- Made `ArrayStringStoreMicroSeedRoute` fields owner-private.
- Made `ArrayStringStoreMicroSeedProof` owner-private and removed its root
  `crate::mir` re-export.
- Added stable route accessors for JSON emission and exact-seed selection:
  - `seed()`
  - `seed_len()`
  - `size()`
  - `ops()`
  - `suffix()`
  - `store_len()`
  - `next_text_window_start()`
  - `next_text_window_len()`
  - `proof()`
- Added an owner-local `cfg(test)` fixture builder for cross-module tests.
- Updated JSON emission and exact-seed selection to consume accessors only.
- Preserved route detection, JSON field names/values, exact-seed priority,
  `.inc` behavior, helper symbols, and lowering.

Verification:

```bash
cargo check -q
cargo test -q array_string_store_micro_seed
cargo test -q exact_seed_backend_route
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

Note: quick gate still reports the existing chip8 release artifact sync
warning, then completes with `[dev-gate] profile=quick ok`.
