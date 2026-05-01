---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: centralize LoweringPlan v0 need-kind rules without adding a shape.
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P80-LOWERING-PLAN-MAPGET-DIRECTABI-CONSUME.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
---

# P81 LoweringPlan Need Kind Table SSOT

## Goal

Keep `mir_call_need` from becoming a new raw matcher ladder as more
LoweringPlan slices land.

P72-P80 added one accepted shape per card. That was correct for proving the
contract, but the plan-to-need mapping is now the one place where shape rows are
starting to accumulate as repeated `strcmp` blocks.

## Decision

- Replace the LoweringPlan need-kind `if` ladder with a small table of proven
  plan rows.
- Keep the same accepted rows and helper symbols.
- Keep legacy `generic_method_routes` fallback unchanged.
- Do not add a new CoreOp, route, or fixture in this card.

## Non-goals

- no new accepted LoweringPlan shape
- no route-policy or emit-policy behavior change
- no legacy route metadata removal
- no helper declaration changes

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
for f in \
  apps/tests/mir_shape_guard/lowering_plan_runtime_data_map_get_min_v1.mir.json \
  apps/tests/mir_shape_guard/lowering_plan_map_has_directabi_min_v1.mir.json \
  apps/tests/mir_shape_guard/lowering_plan_map_len_directabi_min_v1.mir.json \
  apps/tests/mir_shape_guard/lowering_plan_array_len_directabi_min_v1.mir.json \
  apps/tests/mir_shape_guard/lowering_plan_string_len_directabi_min_v1.mir.json \
  apps/tests/mir_shape_guard/lowering_plan_array_get_directabi_min_v1.mir.json \
  apps/tests/mir_shape_guard/lowering_plan_map_get_directabi_min_v1.mir.json
do
  NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
    --in "$f" \
    --out "/tmp/$(basename "$f" .mir.json).o"
done
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
