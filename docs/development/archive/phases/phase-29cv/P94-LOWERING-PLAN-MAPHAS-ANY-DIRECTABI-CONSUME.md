---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: consume LoweringPlan v0 for MapHas any-key DirectAbi.
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - apps/tests/mir_shape_guard/lowering_plan_map_has_any_directabi_min_v1.mir.json
---

# P94 LoweringPlan MapHas Any DirectAbi Consume

## Goal

Add the next single LoweringPlan v0 accepted map shape: `MapHas` with an
unknown/handle key, lowered through `nyash.map.probe_hh`.

## Decision

- Add a plan-only `MapHas` fixture with `route_kind=map_contains_any`.
- Reuse the existing LoweringPlan need-kind, route-state, and has-policy rows.
- Keep `MapHas` i64 and any-key variants as separate proven slices.

## Non-goals

- no map lookup fusion change
- no `RuntimeDataBox.has` facade plan
- no HotInline/perf keeper claim

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_map_has_any_directabi_min_v1.mir.json \
  --out /tmp/p94_lowering_plan_map_has_any_directabi.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_map_has_directabi_min_v1.mir.json \
  --out /tmp/p94_regress_lowering_plan_map_has_i64_directabi.o
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
