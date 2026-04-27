---
Status: Landed
Date: 2026-04-27
Scope: Make ArrayTextEditRoute field layout owner-private
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-505-array-text-loopcarry-route-field-boundary-card.md
  - src/mir/array_text_edit_plan.rs
  - src/mir/array_text_combined_region_plan.rs
  - src/mir/mod.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-506: Array Text Edit Route Field Boundary

## Goal

Keep the active array/text edit route field layout and proof vocabulary owned by
`array_text_edit_plan`.

The route remains public function metadata, but combined-region derivation and
JSON emission should consume a stable route view instead of reading edit fields
or matching edit proof enums directly.

## Inventory

Current external consumers:

- `src/mir/array_text_combined_region_plan.rs`
  - matches edit kind, split policy, and proof vocabulary directly
  - reads edit block/value/text fields directly while deriving combined regions
- `src/runner/mir_json_emit/root.rs`
  - reads edit route fields directly for JSON metadata
- `src/mir/mod.rs`
  - root-exports the edit route plus edit proof vocabulary

Owner-local consumers:

- `src/mir/array_text_edit_plan.rs`
  - owns detection, edit payload derivation, split-policy vocabulary, and proof vocabulary

## Cleaner Boundary

```text
array_text_edit_plan
  owns route fields, split-policy vocabulary, and proof vocabulary
  exposes stable read accessors and owner predicates

combined-region / JSON / tests
  consume accessors and predicates only
```

## Boundaries

- BoxShape-only.
- Do not change edit route detection.
- Do not change combined-region derivation semantics.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.
- Do not add new edit route variants.

## Acceptance

- `ArrayTextEditRoute` fields are private.
- `ArrayTextEditKind`, `ArrayTextEditSplitPolicy`, and `ArrayTextEditProof` are owner-private and no longer root-exported.
- Combined-region derivation reads edit metadata through route accessors and owner predicates.
- JSON emission and focused tests read through route accessors.
- `cargo check -q` passes.
- Focused array text edit/combined-region tests pass.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.

## Result

- Made `ArrayTextEditRoute` field layout owner-private.
- Kept edit metadata public through stable accessors and owner predicates only.
- Made `ArrayTextEditKind`, `ArrayTextEditSplitPolicy`, and `ArrayTextEditProof` owner-private and removed their root exports.
- Moved combined-region derivation away from direct edit proof/field reads.
- Moved JSON emission and focused route tests to the route accessor API.
- Preserved edit route detection, combined-region semantics, JSON field names/values, `.inc` behavior, helper symbols, and lowering.

## Verification

```bash
cargo fmt
cargo check -q
cargo test -q detects_lenhalf_insert_mid_same_slot_edit_route
cargo test -q benchmark_kilo_kernel_small_has_combined_edit_observer_region
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
