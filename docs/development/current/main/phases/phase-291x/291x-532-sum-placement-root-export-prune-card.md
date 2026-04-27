---
Status: Landed
Date: 2026-04-27
Scope: Prune sum-placement semantic metadata root exports
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/mod.rs
  - src/mir/agg_local_scalarization.rs
  - src/mir/placement_effect.rs
  - src/mir/printer.rs
  - src/mir/sum_placement_selection.rs
  - src/runner/json_v0_bridge/tests.rs
  - src/runner/mir_json_emit/tests/placement.rs
---

# 291x-532: Sum-Placement Root Export Prune

## Goal

Keep sum-placement semantic metadata owned by `sum_placement`,
`sum_placement_layout`, and `sum_placement_selection` instead of the broad MIR
root.

The MIR root should expose refresh entry points for orchestration. Consumers
that construct or inspect the vocabulary should import the owner modules
directly.

## Inventory

Removed root exports:

- `SumObjectizationBarrier`
- `SumPlacementFact`
- `SumPlacementState`
- `SumLocalAggregateLayout`
- `SumPlacementLayout`
- `SumPlacementPath`
- `SumPlacementSelection`

Migrated consumers:

- `src/mir/agg_local_scalarization.rs`
- `src/mir/placement_effect.rs`
- `src/mir/printer.rs`
- `src/mir/sum_placement_selection.rs`
- `src/runner/json_v0_bridge/tests.rs`
- `src/runner/mir_json_emit/tests/placement.rs`

Existing owner imports already covered:

- `src/mir/function/types.rs`
- sum-variant seed owner modules.

## Cleaner Boundary

```text
sum_placement / sum_placement_layout / sum_placement_selection
  own SumPlacement* vocabulary

mir root
  exports refresh_function_sum_placement_*
  exports refresh_module_sum_placement_*
```

## Boundaries

- BoxShape-only.
- Do not change sum-placement fact, selection, or layout derivation.
- Do not change thin-entry behavior.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.

## Acceptance

- MIR root no longer re-exports `SumPlacement*` vocabulary.
- Consumers use owner modules for sum-placement vocabulary.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed sum-placement vocabulary from the MIR root export surface.
- Kept sum-placement refresh entry points available at the MIR root.
- Preserved route metadata and JSON output.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
