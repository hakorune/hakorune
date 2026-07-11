---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: consume LoweringPlan v0 for StringIndexOf DirectAbi.
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - apps/tests/mir_shape_guard/lowering_plan_string_indexof_directabi_min_v1.mir.json
---

# P93 LoweringPlan StringIndexOf DirectAbi Consume

## Goal

Add the next single LoweringPlan v0 accepted string shape:
`StringIndexOf`, lowered through `nyash.string.indexOf_hh`.

## Decision

- Add a plan-only `StringIndexOf` fixture with no `generic_method_routes`.
- Add one `StringIndexOf` row to the LoweringPlan need-kind table.
- Reuse the existing plan-first route-state selection and `indexOf` emitter.

## Non-goals

- no array-string deferred `indexOf` observer change
- no text-state residence route change
- no HotInline/perf keeper claim

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_string_indexof_directabi_min_v1.mir.json \
  --out /tmp/p93_lowering_plan_string_indexof_directabi.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_string_substring_directabi_min_v1.mir.json \
  --out /tmp/p93_regress_lowering_plan_string_substring_directabi.o
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
