---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: consume LoweringPlan v0 for ArrayLen DirectAbi.
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P75-LOWERING-PLAN-MAPLEN-DIRECTABI-CONSUME.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - apps/tests/mir_shape_guard/lowering_plan_array_len_directabi_min_v1.mir.json
---

# P76 LoweringPlan ArrayLen DirectAbi Consume

## Goal

Add the next single LoweringPlan v0 accepted shape: `ArrayLen` lowered through
the direct ABI helper `nyash.array.slot_len_h`.

This mirrors P75, but keeps the array length shape as its own fixture and
commit.

## Decision

- Add a plan-only `ArrayLen` fixture with no `generic_method_routes`.
- Map a valid `LoweringPlan` view for `generic_method.len` / `ArrayLen` /
  `array_slot_len` to the existing `ARRAY_LEN` need kind.
- Keep legacy route metadata as migration fallback.

## Non-goals

- no `StringLen` plan-only fixture
- no len surface widening
- no helper-symbol inference from raw MIR
- no perf keeper claim

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_array_len_directabi_min_v1.mir.json \
  --out /tmp/p76_lowering_plan_array_len_directabi.o
NYASH_LLVM_ROUTE_TRACE=1 bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_length_min.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
