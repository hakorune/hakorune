# P381EO Stage0 Global-Call Target-Shape Read Prune

Date: 2026-05-06
Scope: remove the unused Stage0 C read of the retired direct global-call target shape.

## Context

The temporary global-call capsules have been retired from the Stage0
`target_shape` inventory. Direct global-call lowering now consumes MIR-owned
facts: proof, return shape, result origin, definition owner, and trace consumer.

`LoweringPlanGlobalCallView` still carried a `target_shape` field even though no
Stage0 code read it after the capsule exits.

## Change

Removed the `target_shape` member and JSON read from
`hako_llvmc_ffi_lowering_plan_metadata.inc`.

Preserved the unsupported-route diagnostic breadcrumbs:

- `target_shape_reason`
- `target_shape_blocker_symbol`
- `target_shape_blocker_reason`

Those fields are still used only when reporting why an unsupported direct
global-call route could not lower.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/stage0_shape_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Stage0 no longer stores the obsolete target-shape value in its global-call
metadata view while keeping failure diagnostics intact.
