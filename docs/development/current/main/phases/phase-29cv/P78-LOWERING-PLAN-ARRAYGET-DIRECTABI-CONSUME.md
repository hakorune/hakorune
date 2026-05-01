---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: consume LoweringPlan v0 for ArrayGet DirectAbi.
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P77-LOWERING-PLAN-STRINGLEN-DIRECTABI-CONSUME.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - apps/tests/mir_shape_guard/lowering_plan_array_get_directabi_min_v1.mir.json
---

# P78 LoweringPlan ArrayGet DirectAbi Consume

## Goal

Add the next single LoweringPlan v0 accepted shape: `ArrayGet` lowered through
the direct ABI helper `nyash.array.slot_load_hi`.

## Decision

- Add a plan-only `ArrayGet` fixture with no `generic_method_routes`.
- Map a valid `LoweringPlan` view for `generic_method.get` / `ArrayGet` /
  `array_slot_load_any` to the existing `ARRAY_GET` need kind.
- Keep legacy route metadata as migration fallback.

## Non-goals

- no `MapGet DirectAbi` plan-only fixture
- no array get surface widening
- no helper-symbol inference from raw MIR
- no perf keeper claim

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_array_get_directabi_min_v1.mir.json \
  --out /tmp/p78_lowering_plan_array_get_directabi.o
NYASH_LLVM_ROUTE_TRACE=1 bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_get_min.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
