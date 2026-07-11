---
Status: Landed
Date: 2026-04-27
Scope: Prune placement-effect semantic metadata root exports
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/mod.rs
  - src/mir/passes/string_corridor_sink/shared.rs
  - src/runner/mir_json_emit/placement_effect.rs
  - src/runner/mir_json_emit/tests/placement.rs
---

# 291x-525: Placement-Effect Root Export Prune

## Goal

Keep placement/effect semantic metadata owned by `placement_effect` instead of
the broad MIR root.

The root should expose refresh entry points for pass orchestration, but
semantic vocabulary consumers should import the owner module directly.

## Inventory

Removed root exports:

- `PlacementEffectRoute`
- `PlacementEffectSource`
- `PlacementEffectDecision`
- `PlacementEffectState`
- `PlacementEffectDemand`
- `PlacementEffectPublicationBoundary`
- `PlacementEffectBorrowContract`
- `PlacementEffectStringProof`

Migrated consumers:

- `src/mir/passes/string_corridor_sink/shared.rs`
- `src/runner/mir_json_emit/placement_effect.rs`
- `src/runner/mir_json_emit/tests/placement.rs`

Existing owner imports already covered:

- `src/mir/function/types.rs`

## Cleaner Boundary

```text
placement_effect
  owns PlacementEffect* semantic vocabulary

mir root
  exports refresh_function_placement_effect_routes
  exports refresh_module_placement_effect_routes
```

## Boundaries

- BoxShape-only.
- Do not change placement/effect route collection.
- Do not change string-corridor sink decisions.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.

## Acceptance

- MIR root no longer re-exports `PlacementEffect*` vocabulary.
- Internal consumers use `crate::mir::placement_effect`.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed placement/effect semantic vocabulary from the MIR root export
  surface.
- Kept placement/effect refresh entry points available at the MIR root.
- Preserved sink behavior and JSON output.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
