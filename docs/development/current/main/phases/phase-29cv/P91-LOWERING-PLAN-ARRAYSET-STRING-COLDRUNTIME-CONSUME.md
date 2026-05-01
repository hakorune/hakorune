---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: consume LoweringPlan v0 for ArraySet string ColdRuntime.
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P90-LOWERING-PLAN-SET-ROUTE-VALUE-SHAPE-GUARD.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - apps/tests/mir_shape_guard/lowering_plan_array_set_string_coldruntime_min_v1.mir.json
---

# P91 LoweringPlan ArraySet String ColdRuntime Consume

## Goal

Add the next single LoweringPlan v0 accepted set shape: `ArraySet` with an
i64 index and StringBox value, lowered through `nyash.array.set_his`.

## Decision

- Add a plan-only `ArraySet` string fixture with no `generic_method_routes`.
- Add one `ArraySet` string row to the LoweringPlan need-kind table.
- Add one `ArraySet` string row to the set-route table.
- Use the existing string-public store helper; do not add kernel slot behavior.

## Non-goals

- no kernel-slot string store plan
- no array-string window optimization change
- no DirectAbi/perf keeper claim

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_array_set_string_coldruntime_min_v1.mir.json \
  --out /tmp/p91_lowering_plan_array_set_string_coldruntime.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_array_set_handle_coldruntime_min_v1.mir.json \
  --out /tmp/p91_regress_lowering_plan_array_set_handle_coldruntime.o
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
