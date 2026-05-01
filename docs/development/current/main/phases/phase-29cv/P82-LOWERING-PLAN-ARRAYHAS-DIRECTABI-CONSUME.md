---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: consume LoweringPlan v0 for ArrayHas DirectAbi.
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P81-LOWERING-PLAN-NEED-KIND-TABLE-SSOT.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_has_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - apps/tests/mir_shape_guard/lowering_plan_array_has_directabi_min_v1.mir.json
---

# P82 LoweringPlan ArrayHas DirectAbi Consume

## Goal

Add the next single LoweringPlan v0 accepted shape: `ArrayHas` lowered through
the direct ABI helper `nyash.array.has_hh`.

## Decision

- Add a plan-only `ArrayHas` fixture with no `generic_method_routes`.
- Allow the has policy plan reader to accept `ArrayHas` in addition to `MapHas`.
- Add one `ArrayHas` row to the LoweringPlan need-kind table.
- Keep legacy route metadata as migration fallback.

## Non-goals

- no route-state field for array-has
- no runtime-data has facade change
- no map-has behavior change
- no perf keeper claim

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_array_has_directabi_min_v1.mir.json \
  --out /tmp/p82_lowering_plan_array_has_directabi.o
NYASH_LLVM_ROUTE_TRACE=1 bash tools/smokes/v2/profiles/integration/phase29ck_boundary/array/phase29ck_boundary_pure_array_has_min.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
