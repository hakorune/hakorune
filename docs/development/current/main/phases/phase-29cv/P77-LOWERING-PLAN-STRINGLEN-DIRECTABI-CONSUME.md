---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: consume LoweringPlan v0 for StringLen DirectAbi.
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P76-LOWERING-PLAN-ARRAYLEN-DIRECTABI-CONSUME.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - apps/tests/mir_shape_guard/lowering_plan_string_len_directabi_min_v1.mir.json
---

# P77 LoweringPlan StringLen DirectAbi Consume

## Goal

Add the next single LoweringPlan v0 accepted shape: `StringLen` lowered through
the direct ABI helper `nyash.string.len_h`.

This completes the initial `generic_method.len` direct-ABI trio after P75
`MapLen` and P76 `ArrayLen`.

## Decision

- Add a plan-only `StringLen` fixture with no `generic_method_routes`.
- Map a valid `LoweringPlan` view for `generic_method.len` / `StringLen` /
  `string_len` to the existing `STRING_LEN` need kind.
- Keep legacy route metadata as migration fallback.

## Non-goals

- no substring/window route changes
- no array-string len route changes
- no helper-symbol inference from raw MIR
- no perf keeper claim

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_string_len_directabi_min_v1.mir.json \
  --out /tmp/p77_lowering_plan_string_len_directabi.o
NYASH_LLVM_ROUTE_TRACE=1 bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_string_length_min.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
