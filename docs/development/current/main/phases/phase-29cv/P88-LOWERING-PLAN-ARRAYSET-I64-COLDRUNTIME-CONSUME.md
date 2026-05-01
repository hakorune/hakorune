---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: consume LoweringPlan v0 for ArraySet i64 ColdRuntime.
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P87-LOWERING-PLAN-SET-ROUTE-TABLE-SSOT.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - apps/tests/mir_shape_guard/lowering_plan_array_set_i64_coldruntime_min_v1.mir.json
---

# P88 LoweringPlan ArraySet I64 ColdRuntime Consume

## Goal

Add the next single LoweringPlan v0 accepted set shape: `ArraySet` with an
i64 index and i64 value, lowered through `nyash.array.slot_store_hii`.

## Decision

- Add a plan-only `ArraySet` i64 fixture with no `generic_method_routes`.
- Add one `ArraySet` i64 row to the LoweringPlan need-kind table.
- Add one `ArraySet` i64 row to the set-route table.
- Keep handle/string/publication variants out of this card.

## Non-goals

- no `ArraySet` handle fixture
- no `ArraySet` string fixture
- no array-string publication or promotion change
- no DirectAbi/perf keeper claim

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_array_set_i64_coldruntime_min_v1.mir.json \
  --out /tmp/p88_lowering_plan_array_set_i64_coldruntime.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_map_set_coldruntime_min_v1.mir.json \
  --out /tmp/p88_regress_lowering_plan_map_set_coldruntime.o
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
