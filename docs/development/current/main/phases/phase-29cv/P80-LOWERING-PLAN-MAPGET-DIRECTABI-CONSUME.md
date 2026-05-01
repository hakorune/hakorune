---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: consume LoweringPlan v0 for MapGet DirectAbi.
Related:
  - docs/development/current/main/phases/phase-29cv/P79-LOWERING-PLAN-DIRECTABI-MAPGET-LOCK.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - apps/tests/mir_shape_guard/lowering_plan_map_get_directabi_min_v1.mir.json
---

# P80 LoweringPlan MapGet DirectAbi Consume

## Goal

Add the locked P79 direct-ABI `MapGet` plan-only shape.

P70 already covers `MapGet` as `ColdRuntime`. This card covers the direct
`MapBox.get` route with `nyash.map.slot_load_hh`.

## Decision

- Add a plan-only `MapGet DirectAbi` fixture with no `generic_method_routes`.
- Map a valid `LoweringPlan` view for `generic_method.get` / `MapGet` /
  `map_load_any` to the existing `MAP_GET` need kind.
- Keep legacy route metadata as migration fallback.

## Non-goals

- no `RuntimeDataBox.get` behavior change
- no scalar proof promotion
- no map set / value publication changes
- no perf keeper claim

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_map_get_directabi_min_v1.mir.json \
  --out /tmp/p80_lowering_plan_map_get_directabi.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_runtime_data_map_get_min_v1.mir.json \
  --out /tmp/p80_regress_lowering_plan_runtime_data_map_get.o
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
