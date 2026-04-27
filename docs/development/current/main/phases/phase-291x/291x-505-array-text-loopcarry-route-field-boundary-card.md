---
Status: Landed
Date: 2026-04-27
Scope: Make ArrayTextLoopCarryLenStoreRoute field layout owner-private
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-504-array-string-len-window-route-field-boundary-card.md
  - src/mir/array_text_loopcarry_plan.rs
  - src/mir/array_text_residence_session_plan.rs
  - src/mir/mod.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-505: Array Text Loopcarry Route Field Boundary

## Goal

Keep the active array/text loopcarry len-store route field layout and proof
vocabulary owned by `array_text_loopcarry_plan`.

The route remains public as function metadata, but downstream residence-session
derivation and JSON emission should consume a stable route view instead of
reading fields directly.

## Inventory

Current external consumers:

- `src/mir/array_text_residence_session_plan.rs`
  - derives session routes from loopcarry route fields directly
  - copies route payload fields into the session route
- `src/runner/mir_json_emit/root.rs`
  - reads route fields directly for JSON metadata
- `src/mir/passes/string_corridor_sink/tests/benchmarks.rs`
  - reads route fields directly in benchmark route checks
- `src/mir/mod.rs`
  - root-exports the route record plus proof vocabulary

Owner-local consumers:

- `src/mir/array_text_loopcarry_plan.rs`
  - owns detection, route field derivation, proof vocabulary, and route tests

## Cleaner Boundary

```text
array_text_loopcarry_plan
  owns route fields and proof vocabulary
  exposes stable read accessors

residence session / JSON / tests
  consume accessors only
```

## Boundaries

- BoxShape-only.
- Do not change loopcarry route detection.
- Do not change residence-session derivation semantics.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.
- Do not add new loopcarry route variants.

## Acceptance

- `ArrayTextLoopCarryLenStoreRoute` fields are private.
- `ArrayTextLoopCarryLenStoreProof` is owner-private and no longer root-exported.
- Residence-session derivation reads through route accessors.
- JSON emission and focused tests read through route accessors.
- `cargo check -q` passes.
- Loopcarry/residence focused tests pass.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.

## Result

- Made `ArrayTextLoopCarryLenStoreRoute` field layout owner-private.
- Kept route metadata public through stable accessors only.
- Made `ArrayTextLoopCarryLenStoreProof` owner-private and removed the root export.
- Moved residence-session derivation, JSON emission, and focused benchmark checks to the route accessor API.
- Added `covered_instruction_indices()` so residence-session coverage consumes the owner-provided route window view.
- Preserved loopcarry route detection, residence-session semantics, JSON field names/values, `.inc` behavior, helper symbols, and lowering.

## Verification

```bash
cargo fmt
cargo check -q
cargo test -q array_text_loopcarry
cargo test -q benchmark_meso_substring_concat_array_set_loopcarry_has_len_store_route
cargo test -q array_text_residence
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

Notes:

- `array_text_loopcarry` and `array_text_residence` filters currently match no Rust tests; the benchmark route check is the focused executable coverage for this card.
- `tools/checks/dev_gate.sh quick` still reports the known chip8 release artifact sync warning, but the quick gate completes successfully.
