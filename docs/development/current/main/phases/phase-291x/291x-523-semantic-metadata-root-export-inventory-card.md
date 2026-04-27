---
Status: Landed
Date: 2026-04-27
Scope: Inventory remaining MIR root exports for semantic metadata families
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/mod.rs
---

# 291x-523: Semantic Metadata Root Export Inventory

## Goal

Keep the MIR root cleanup moving without pruning coupled semantic metadata
families blindly.

The route/seed/window root-export cleanup is mostly closed. The remaining broad
exports are not just temporary route selectors; many are semantic metadata APIs
used by MIR refresh passes, JSON emitters, verification, printer diagnostics,
and tests. The next work should therefore split by owner family rather than
remove everything in one pass.

## Inventory

Root exports that remain and are still worth pruning in small cards:

- `AggLocalScalarizationKind` / `AggLocalScalarizationRoute`
- `PlacementEffect*`
- `StringCorridor*`
- `StringKernelPlan*`
- `ThinEntry*`
- `SumPlacement*`
- `StorageClass`
- `ValueConsumerFacts`

Root exports that should remain for now as the MIR public API:

- `MirFunction` / `MirModule` / `MirInstruction`
- `ValueId` / `LocalId`
- `MirType` / `ConstValue` / core ops
- `Effect` / `EffectMask`
- compiler/builder/printer/query/verifier entry points

## Coupled Surfaces

Current root-path consumers show these clusters:

- Agg-local placement:
  - `src/mir/placement_effect.rs`
  - `src/runner/mir_json_emit/agg_local.rs`
  - `src/runner/mir_json_emit/tests/placement.rs`
- Placement effect:
  - `src/mir/passes/string_corridor_sink/shared.rs`
  - `src/runner/mir_json_emit/placement_effect.rs`
  - `src/runner/mir_json_emit/tests/placement.rs`
- String corridor and string kernel:
  - `src/mir/placement_effect.rs`
  - `src/mir/string_kernel_plan.rs`
  - `src/mir/verification/string_kernel.rs`
  - `src/runner/mir_json_emit/plans.rs`
  - `src/runner/mir_json_emit/tests/string_corridor.rs`
- Thin-entry and sum-placement diagnostics:
  - `src/mir/printer.rs`
  - `src/runner/json_v0_bridge/tests.rs`
  - `src/runner/mir_json_emit/tests/thin_entry.rs`
  - `src/runner/mir_json_emit/tests/placement.rs`

## Cleaner Boundary

```text
owner module
  owns semantic metadata structs/enums

mir root
  keeps refresh entry points and stable MIR API

internal consumers/tests
  import semantic metadata through owner modules
```

## Next Cards

1. Prune agg-local root exports first.
2. Prune placement-effect root exports after migrating JSON/pass/test imports.
3. Split string corridor and string kernel into separate cards; they share
   publication/borrow vocabulary and should not be changed together blindly.
4. Prune thin-entry and sum-placement after printer/test fixture constructors
   move to owner-module imports.

## Boundaries

- BoxShape-only.
- Do not change route/fact refresh behavior.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.
- Do not remove root exports for fundamental MIR types in this lane.

## Acceptance

- Remaining root exports are classified by owner family.
- Next pruning order is explicit.
- Current-state pointer targets this inventory.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `git diff --check` passes.

## Result

- Fixed the next root-export cleanup order before touching larger semantic
  metadata surfaces.
- Kept implementation unchanged.

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
