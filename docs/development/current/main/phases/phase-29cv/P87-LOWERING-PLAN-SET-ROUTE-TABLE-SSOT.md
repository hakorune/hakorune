---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: table the LoweringPlan v0 generic-method set-route consumer.
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P85-LOWERING-PLAN-MAPSET-COLDRUNTIME-CONSUME.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc
---

# P87 LoweringPlan Set Route Table SSOT

## Goal

Move the plan-first `generic_method.set` route selection to a table before
adding more set variants.

## Decision

- Add a `LoweringPlanSetRouteRule` table.
- Keep only the already-proven `MapSet` row in this card.
- Preserve the legacy `generic_method_routes` fallback.
- Do not add a new accepted CoreOp shape.

## Non-goals

- no `ArraySet` fixture
- no array-string publication change
- no need-kind table change
- no legacy route fallback rewrite

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_map_set_coldruntime_min_v1.mir.json \
  --out /tmp/p87_lowering_plan_map_set_coldruntime.o
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
