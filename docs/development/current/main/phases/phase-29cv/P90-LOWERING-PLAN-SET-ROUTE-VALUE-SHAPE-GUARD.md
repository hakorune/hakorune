---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: guard LoweringPlan set-route rows by observed value shape.
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P88-LOWERING-PLAN-ARRAYSET-I64-COLDRUNTIME-CONSUME.md
  - docs/development/current/main/phases/phase-29cv/P89-LOWERING-PLAN-ARRAYSET-HANDLE-COLDRUNTIME-CONSUME.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc
---

# P90 LoweringPlan Set Route Value Shape Guard

## Goal

Make plan-first `generic_method.set` route rows validate the observed value
shape before selecting a concrete store helper.

## Decision

- Add a small set-route value-shape enum to the plan table.
- Keep `MapSet` as any-value.
- Require the P88 `ArraySet` i64 row to see an i64 value.
- Require the P89 `ArraySet` handle row to see a non-i64, non-string value.
- Do not add a new accepted CoreOp shape in this card.

## Non-goals

- no `ArraySet` string fixture
- no array-string publication or promotion change
- no need-kind table change

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_array_set_i64_coldruntime_min_v1.mir.json \
  --out /tmp/p90_lowering_plan_array_set_i64_coldruntime.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_array_set_handle_coldruntime_min_v1.mir.json \
  --out /tmp/p90_lowering_plan_array_set_handle_coldruntime.o
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
