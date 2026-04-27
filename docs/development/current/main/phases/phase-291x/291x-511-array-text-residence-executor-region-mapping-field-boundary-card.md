---
Status: Landed
Date: 2026-04-27
Scope: Make ArrayTextResidenceLoopRegionMapping fields owner-private
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-508-array-text-residence-session-route-field-boundary-card.md
  - docs/development/current/main/phases/phase-291x/291x-510-array-text-observer-executor-contract-field-boundary-card.md
  - src/mir/array_text_residence_session_plan.rs
  - src/runner/mir_json_emit/root.rs
  - src/mir/passes/string_corridor_sink/tests/benchmarks.rs
---

# 291x-511: Array Text Residence Executor Region Mapping Field Boundary

## Goal

Keep the array/text residence executor loop-region mapping owned by
`array_text_residence_session_plan`.

Residence sessions expose a nested executor contract for metadata consumers, but
MIR JSON emission and focused benchmark tests should consume mapping accessors
instead of reading mapping fields directly.

## Inventory

Current external consumers:

- `src/runner/mir_json_emit/root.rs`
  - serializes residence executor loop-region mapping into MIR JSON metadata
- `src/mir/passes/string_corridor_sink/tests/benchmarks.rs`
  - verifies residence executor loop-region mapping values in the focused kilo benchmark

Owner-local consumer:

- `src/mir/array_text_residence_session_plan.rs`
  - owns residence executor vocabulary, contract construction, and loop-region mapping derivation

## Cleaner Boundary

```text
array_text_residence_session_plan
  owns executor vocabulary and loop-region mapping fields
  exposes stable read accessors

JSON / focused tests
  consume accessors only
```

## Boundaries

- BoxShape-only.
- Do not change residence session route detection.
- Do not change residence executor contract values.
- Do not change residence loop-region mapping values.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.
- Do not change observer executor contract or mapping fields in this card.

## Acceptance

- Residence executor vocabulary remains owner-local.
- `ArrayTextResidenceLoopRegionMapping` fields are private.
- MIR JSON emission reads residence executor mapping through accessors.
- Focused benchmark tests read residence executor mapping through accessors.
- `cargo check -q` passes.
- Focused residence/combined benchmark tests pass.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.

## Result

- Made residence executor vocabulary owner-private inside `array_text_residence_session_plan`.
- Made `ArrayTextResidenceLoopRegionMapping` field layout owner-private.
- Added stable read accessors for residence loop-region mapping values.
- Moved MIR JSON emission and focused benchmark tests to mapping accessors.
- Preserved residence route detection, executor contract values, mapping values, JSON field names/values, `.inc` behavior, helper symbols, and lowering.

## Verification

```bash
cargo fmt
cargo check -q
cargo test -q benchmark_meso_substring_concat_array_set_loopcarry_has_len_store_route
cargo test -q benchmark_kilo_kernel_small_has_combined_edit_observer_region
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
