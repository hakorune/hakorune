---
Status: Landed
Date: 2026-04-27
Scope: Prune unused MIR root exports for array/text route types
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-505-array-text-loopcarry-route-field-boundary-card.md
  - docs/development/current/main/phases/phase-291x/291x-506-array-text-edit-route-field-boundary-card.md
  - docs/development/current/main/phases/phase-291x/291x-507-array-text-combined-region-route-field-boundary-card.md
  - docs/development/current/main/phases/phase-291x/291x-508-array-text-residence-session-route-field-boundary-card.md
  - docs/development/current/main/phases/phase-291x/291x-509-array-text-observer-route-field-boundary-card.md
  - src/mir/mod.rs
---

# 291x-517: Array Text Route Root Export Prune

## Goal

Keep array/text route type names owned by their route modules instead of the
broad MIR root.

The route types remain public through their owner modules because MIR function
metadata carries them. The root-level convenience exports were not used by
internal consumers.

## Inventory

Removed root exports:

- `ArrayTextLoopCarryLenStoreRoute`
- `ArrayTextEditRoute`
- `ArrayTextCombinedRegionRoute`
- `ArrayTextResidenceSessionRoute`
- `ArrayTextObserverRoute`

Current consumers:

- `src/mir/function/types.rs` imports all route types through owner module paths.
- Route planners use sibling owner module paths directly.
- No code consumes these route types through `crate::mir::...`.

## Cleaner Boundary

```text
array_text_*_plan
  owns route type names

mir root
  exports refresh entry points only for these array/text lanes
```

## Boundaries

- BoxShape-only.
- Do not change route detection.
- Do not change route metadata values.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.

## Acceptance

- MIR root no longer re-exports the array/text route types listed above.
- Owner-module route type imports continue to compile.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.

## Result

- Removed unused root-level convenience exports for array/text route types.
- Preserved all refresh entry point exports.
- Preserved route metadata, JSON, `.inc` behavior, helper symbols, and lowering.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
