---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: table LoweringPlan v0 generic-method emit-kind selection.
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc
---

# P95 LoweringPlan Generic Emit Kind Table SSOT

## Goal

Move plan-first `generic_method_emit` selection away from a per-op branch
ladder and into table rows keyed by the shared LoweringPlan generic-method
view.

## Decision

- Add a small `LoweringPlanGenericMethodEmitRule` table.
- Match `source_route_id`, `core_op`, `route_kind`, and `tier`.
- Preserve the legacy `generic_method_routes` fallback unchanged.

## Non-goals

- no new accepted shape
- no legacy route metadata cleanup
- no `indexOf` emitter change; `indexOf` remains route-state driven

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
for f in apps/tests/mir_shape_guard/lowering_plan_*_min_v1.mir.json; do
  target/release/ny-llvmc --in "$f" --out "/tmp/$(basename "$f" .mir.json).o"
done
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
