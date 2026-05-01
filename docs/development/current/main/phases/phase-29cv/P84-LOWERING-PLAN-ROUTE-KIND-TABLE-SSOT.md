---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: table the LoweringPlan v0 mir_call route-kind consumer.
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P81-LOWERING-PLAN-NEED-KIND-TABLE-SSOT.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
---

# P84 LoweringPlan Route Kind Table SSOT

## Goal

Move the `mir_call_route` LoweringPlan v0 route-kind selection from a local
`strcmp` ladder to a small table keyed by the shared LoweringPlan view.

## Decision

- Add one `LoweringPlanRouteRule` table for plan-first route-state selection.
- Preserve the legacy `generic_method_routes` reader as migration fallback.
- Keep the matching fields behavior-equivalent to the existing plan reader:
  `source_route_id`, `core_op`, `route_kind`, and `tier`.
- Do not add a new accepted CoreOp shape in this card.

## Non-goals

- no new fixture
- no `ArraySet` / `MapSet` plan consume
- no string substring/indexOf promotion claim
- no legacy route fallback rewrite

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_array_push_coldruntime_min_v1.mir.json \
  --out /tmp/p84_lowering_plan_array_push_coldruntime.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_map_has_directabi_min_v1.mir.json \
  --out /tmp/p84_lowering_plan_map_has_directabi.o
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
