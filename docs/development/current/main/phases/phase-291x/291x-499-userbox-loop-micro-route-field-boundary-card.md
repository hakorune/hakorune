---
Status: Landed
Date: 2026-04-27
Scope: Make UserBoxLoopMicroSeedRoute field layout owner-private
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-498-userbox-local-scalar-route-field-boundary-card.md
  - src/mir/userbox_loop_micro_seed_plan.rs
  - src/mir/exact_seed_backend_route.rs
  - src/mir/mod.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-499: UserBox Loop Micro Route Field Boundary

## Goal

Keep the UserBox loop-micro exact seed route record owned by
`userbox_loop_micro_seed_plan`.

The route remains public as function metadata, and the route kind remains public
vocabulary, but the field layout and route-local proof enum should not be the
consumer contract.

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

- `src/mir/userbox_loop_micro_seed_plan.rs`
  - owns detection, route field derivation, proof vocabulary, and tests

## Cleaner Boundary

```text
userbox_loop_micro_seed_plan
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

- `UserBoxLoopMicroSeedRoute` fields are private.
- `UserBoxLoopMicroSeedProof` is owner-private and no longer root-exported.
- JSON emitter and exact-seed selector read through accessors.
- Cross-module test construction uses an owner `cfg(test)` fixture.
- `cargo check -q` passes.
- UserBox loop micro and exact-seed focused tests pass.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.

## Result

Landed as a BoxShape-only cleanup.

- `UserBoxLoopMicroSeedRoute` now keeps its fields owner-private.
- `UserBoxLoopMicroSeedProof` is owner-private and no longer root-exported.
- JSON emission and exact-seed selection consume route accessors only.
- Cross-module test construction uses the owner `cfg(test)` fixture.
- No `.inc`, helper symbol, lowering, route detection, or route priority behavior changed.

Verification:

```bash
cargo check -q
cargo test -q userbox_loop_micro_seed
cargo test -q exact_seed_backend_route
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

Note: the quick gate still reports the pre-existing chip8 release artifact sync
warning, then finishes with `[dev-gate] profile=quick ok`.
