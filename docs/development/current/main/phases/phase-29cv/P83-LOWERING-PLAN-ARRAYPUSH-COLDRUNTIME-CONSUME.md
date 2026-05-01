---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: consume LoweringPlan v0 for ArrayPush ColdRuntime.
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P82-LOWERING-PLAN-ARRAYHAS-DIRECTABI-CONSUME.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - apps/tests/mir_shape_guard/lowering_plan_array_push_coldruntime_min_v1.mir.json
---

# P83 LoweringPlan ArrayPush ColdRuntime Consume

## Goal

Add the next single LoweringPlan v0 accepted shape: `ArrayPush` as explicit
`ColdRuntime` lowering through `nyash.array.slot_append_hh`.

## Decision

- Add a plan-only `ArrayPush` fixture with no `generic_method_routes`.
- Add one `ArrayPush` row to the LoweringPlan need-kind table.
- Reuse the existing route/emit policy for `array_append_any`.
- Keep legacy route metadata as migration fallback.

## Non-goals

- no `ArraySet` / `MapSet` fixture
- no array-string promotion change
- no DirectAbi claim for push
- no perf keeper claim

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_array_push_coldruntime_min_v1.mir.json \
  --out /tmp/p83_lowering_plan_array_push_coldruntime.o
NYASH_LLVM_ROUTE_TRACE=1 bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_push_min.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
