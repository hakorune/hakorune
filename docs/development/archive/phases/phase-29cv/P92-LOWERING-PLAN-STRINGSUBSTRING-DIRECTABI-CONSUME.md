---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: consume LoweringPlan v0 for StringSubstring DirectAbi.
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - apps/tests/mir_shape_guard/lowering_plan_string_substring_directabi_min_v1.mir.json
---

# P92 LoweringPlan StringSubstring DirectAbi Consume

## Goal

Add the next single LoweringPlan v0 accepted string shape:
`StringSubstring`, lowered through `nyash.string.substring_hii`.

## Decision

- Add a plan-only `StringSubstring` fixture with no `generic_method_routes`.
- Add one `StringSubstring` row to the LoweringPlan need-kind table.
- Reuse the existing plan-first route and generic-method substring emitters.

## Non-goals

- no substring-concat route change
- no kernel-slot substring optimization change
- no HotInline/perf keeper claim

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_string_substring_directabi_min_v1.mir.json \
  --out /tmp/p92_lowering_plan_string_substring_directabi.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_string_len_directabi_min_v1.mir.json \
  --out /tmp/p92_regress_lowering_plan_string_len_directabi.o
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
