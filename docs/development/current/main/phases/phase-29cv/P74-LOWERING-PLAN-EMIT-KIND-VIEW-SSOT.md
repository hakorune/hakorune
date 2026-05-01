---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: make generic-method emit-kind LoweringPlan reads use the common view.
Related:
  - docs/development/current/main/phases/phase-29cv/P73-LOWERING-PLAN-CONSUMER-VIEW-SSOT.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc
  - lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_metadata.inc
---

# P74 LoweringPlan Emit Kind View SSOT

## Goal

Finish the first LoweringPlan consumer-view cleanup by moving the generic method
emit-kind classifier onto the shared plan view.

P73 covered route, need, get, and has consumers. The remaining generic-method
entry point is `classify_generic_method_emit_kind_from_lowering_plan`, which
still reads `source`, `source_route_id`, `core_op`, `route_kind`, `proof`, and
`tier` directly.

## Decision

- Use `LoweringPlanGenericMethodView` in the emit-kind classifier.
- Keep the exact accepted emit-kind rows unchanged.
- Do not add `MapLen` / `ArrayLen` plan-only fixtures in this card.

## Non-goals

- no new accepted CoreOp
- no legacy `generic_method_routes` removal
- no helper declaration changes
- no perf keeper claim

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_runtime_data_map_get_min_v1.mir.json \
  --out /tmp/p74_lowering_plan_runtime_data_map_get.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_map_has_directabi_min_v1.mir.json \
  --out /tmp/p74_lowering_plan_map_has_directabi.o
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
