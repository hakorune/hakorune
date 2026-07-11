---
Status: Landed
Date: 2026-04-27
Scope: Prune thin-entry semantic metadata root exports
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/mod.rs
  - src/mir/agg_local_scalarization.rs
  - src/mir/placement_effect.rs
  - src/mir/printer.rs
  - src/mir/sum_placement.rs
  - src/mir/sum_placement_selection.rs
  - src/runner/json_v0_bridge/tests.rs
  - src/runner/mir_json_emit/tests/placement.rs
  - src/runner/mir_json_emit/tests/thin_entry.rs
---

# 291x-531: Thin-Entry Root Export Prune

## Goal

Keep thin-entry semantic metadata owned by `thin_entry` and
`thin_entry_selection` instead of the broad MIR root.

The MIR root should expose refresh entry points for orchestration. Fixture and
test constructors that inspect vocabulary should import the owner modules
directly.

## Inventory

Removed root exports:

- `ThinEntryCandidate`
- `ThinEntryCurrentCarrier`
- `ThinEntryDemand`
- `ThinEntryPreferredEntry`
- `ThinEntrySurface`
- `ThinEntryValueClass`
- `ThinEntrySelection`
- `ThinEntrySelectionState`

Migrated consumers:

- `src/mir/agg_local_scalarization.rs`
- `src/mir/placement_effect.rs`
- `src/mir/printer.rs`
- `src/mir/sum_placement.rs`
- `src/mir/sum_placement_selection.rs`
- `src/runner/json_v0_bridge/tests.rs`
- `src/runner/mir_json_emit/tests/placement.rs`
- `src/runner/mir_json_emit/tests/thin_entry.rs`

Existing owner imports already covered:

- `src/mir/function/types.rs`
- seed/placement owner modules that already imported `thin_entry` directly.

## Cleaner Boundary

```text
thin_entry / thin_entry_selection
  own ThinEntry* vocabulary

mir root
  exports refresh_function_thin_entry_candidates
  exports refresh_module_thin_entry_candidates
  exports refresh_function_thin_entry_selections
  exports refresh_module_thin_entry_selections
```

## Boundaries

- BoxShape-only.
- Do not change thin-entry candidate or selection derivation.
- Do not change sum-placement behavior.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.

## Acceptance

- MIR root no longer re-exports `ThinEntry*` vocabulary.
- Consumers use `crate::mir::thin_entry` or `crate::mir::thin_entry_selection`.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed thin-entry vocabulary from the MIR root export surface.
- Kept thin-entry refresh entry points available at the MIR root.
- Preserved route metadata and JSON output.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
