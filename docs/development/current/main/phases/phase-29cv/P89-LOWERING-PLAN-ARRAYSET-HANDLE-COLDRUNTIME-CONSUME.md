---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: consume LoweringPlan v0 for ArraySet handle ColdRuntime.
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P88-LOWERING-PLAN-ARRAYSET-I64-COLDRUNTIME-CONSUME.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - apps/tests/mir_shape_guard/lowering_plan_array_set_handle_coldruntime_min_v1.mir.json
---

# P89 LoweringPlan ArraySet Handle ColdRuntime Consume

## Goal

Add the next single LoweringPlan v0 accepted set shape: `ArraySet` with an
i64 index and non-string handle value, lowered through
`nyash.array.slot_store_hih`.

## Decision

- Add a plan-only `ArraySet` handle fixture with no `generic_method_routes`.
- Add one `ArraySet` handle row to the LoweringPlan need-kind table.
- Add one `ArraySet` handle row to the set-route table.
- Use a `MapBox` value in the fixture to avoid string publication behavior.

## Non-goals

- no `ArraySet` string fixture
- no array-string publication or promotion change
- no DirectAbi/perf keeper claim

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_array_set_handle_coldruntime_min_v1.mir.json \
  --out /tmp/p89_lowering_plan_array_set_handle_coldruntime.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_array_set_i64_coldruntime_min_v1.mir.json \
  --out /tmp/p89_regress_lowering_plan_array_set_i64_coldruntime.o
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
