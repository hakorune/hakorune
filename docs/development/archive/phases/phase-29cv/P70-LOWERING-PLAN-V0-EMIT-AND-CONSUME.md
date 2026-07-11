---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: emit and consume the first LoweringPlan JSON v0 entries.
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/runner/mir_json_emit/root.rs
  - lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_metadata.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc
---

# P70 LoweringPlan v0 Emit And Consume

## Goal

Land the first non-invasive LoweringPlan path:

- MIR JSON emits `metadata.lowering_plan` from existing `generic_method_routes`
- ny-llvmc reads plan metadata first for `mir_call` CoreMethod sites
- legacy `generic_method_routes` remain fallback while migration is incomplete

## Decision

- v0 covers the generic-method CoreMethod rows already present in MIR metadata.
- This card proves the plan consumer with `MapGet` / `ColdRuntime`.
- The plan-only proof intentionally removes `generic_method_routes` from the
  fixture so the backend cannot pass by the old route reader.

## Non-goals

- no removal of `generic_method_routes`
- no broad plan emitter rewrite
- no new accepted method surface
- no perf keeper promotion

## Acceptance

```bash
cargo test --lib build_mir_json_root_emits_generic_method_routes
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_runtime_data_map_get_min_v1.mir.json \
  --out /tmp/p70_lowering_plan_runtime_data_map_get.o
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
