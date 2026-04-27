---
Status: Landed
Date: 2026-04-27
Scope: Make ArrayTextCombinedRegionRoute field layout owner-private
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-506-array-text-edit-route-field-boundary-card.md
  - src/mir/array_text_combined_region_plan.rs
  - src/mir/mod.rs
  - src/runner/mir_json_emit/root.rs
  - src/mir/passes/string_corridor_sink/tests/benchmarks.rs
---

# 291x-507: Array Text Combined Region Route Field Boundary

## Goal

Keep the array/text combined-region route field layout and proof vocabulary
owned by `array_text_combined_region_plan`.

The route remains public function metadata, but JSON emission and focused tests
should consume stable accessors instead of reading the large combined-region
record directly.

## Inventory

Current external consumers:

- `src/runner/mir_json_emit/root.rs`
  - reads the combined-region record directly for MIR JSON metadata
- `src/mir/passes/string_corridor_sink/tests/benchmarks.rs`
  - reads proof, execution-mode, scalar bounds, text payloads, and PHI values directly
- `src/mir/mod.rs`
  - root-exports combined-region proof vocabulary

Owner-local consumers:

- `src/mir/array_text_combined_region_plan.rs`
  - owns derivation, route field population, proof-region vocabulary, and byte-boundary proof vocabulary

## Cleaner Boundary

```text
array_text_combined_region_plan
  owns route fields and proof vocabulary
  exposes stable read accessors

JSON / focused tests
  consume accessors only
```

## Boundaries

- BoxShape-only.
- Do not change combined-region derivation.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.
- Do not add new combined-region route variants.

## Acceptance

- `ArrayTextCombinedRegionRoute` fields are private.
- Combined-region proof/execution/region/byte-boundary vocabulary is owner-private and no longer root-exported.
- JSON emission and focused benchmark checks read through route accessors.
- `cargo check -q` passes.
- Focused combined-region benchmark test passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.

## Result

- Made `ArrayTextCombinedRegionRoute` field layout owner-private.
- Made combined-region proof, execution-mode, proof-region, and byte-boundary proof vocabulary owner-private and removed their root exports.
- Moved MIR JSON emission to stable combined-region accessors.
- Moved the focused combined-region benchmark check to stable accessors.
- Preserved combined-region derivation, JSON field names/values, `.inc` behavior, helper symbols, and lowering.

## Verification

```bash
cargo fmt
cargo check -q
cargo test -q benchmark_kilo_kernel_small_has_combined_edit_observer_region
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
