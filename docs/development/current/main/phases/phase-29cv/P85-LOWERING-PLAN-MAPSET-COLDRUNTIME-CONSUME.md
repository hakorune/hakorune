---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: consume LoweringPlan v0 for MapSet ColdRuntime.
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P83-LOWERING-PLAN-ARRAYPUSH-COLDRUNTIME-CONSUME.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - apps/tests/mir_shape_guard/lowering_plan_map_set_coldruntime_min_v1.mir.json
---

# P85 LoweringPlan MapSet ColdRuntime Consume

## Goal

Add the next single LoweringPlan v0 accepted shape: `MapSet` as explicit
`ColdRuntime` lowering through `nyash.map.slot_store_hhh`.

## Decision

- Add a plan-only `MapSet` fixture with no `generic_method_routes`.
- Add one `MapSet` row to the LoweringPlan need-kind table.
- Add a plan-first set-route reader for `MapSet` only.
- Keep `ArraySet` out of scope because value-shape and array-string
  publication rules need a separate card.

## Non-goals

- no `ArraySet` fixture
- no array-string promotion change
- no DirectAbi claim for set
- no legacy route fallback rewrite

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_map_set_coldruntime_min_v1.mir.json \
  --out /tmp/p85_lowering_plan_map_set_coldruntime.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_array_push_coldruntime_min_v1.mir.json \
  --out /tmp/p85_regress_lowering_plan_array_push_coldruntime.o
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
