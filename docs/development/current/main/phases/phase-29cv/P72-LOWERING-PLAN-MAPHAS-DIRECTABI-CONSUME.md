---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: consume LoweringPlan v0 for MapHas DirectAbi.
Related:
  - docs/development/current/main/phases/phase-29cv/P71-LOWERING-PLAN-DIRECTABI-MAPHAS-LOCK.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_has_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - apps/tests/mir_shape_guard/lowering_plan_map_has_directabi_min_v1.mir.json
---

# P72 LoweringPlan MapHas DirectAbi Consume

## Goal

Prove the first `DirectAbi` LoweringPlan v0 path with `MapHas`.

## Decision

- Add a plan-only `MapHas` fixture with no `generic_method_routes`.
- Read `metadata.lowering_plan` first in the has policy.
- Read `metadata.lowering_plan` in the mir-call need prepass so DirectAbi
  helper declarations are plan-owned too.
- Preserve legacy `generic_method_routes` fallback during migration.
- Keep the accepted surface identical to the existing `map_contains_i64` row.

## Non-goals

- no `MapHas` surface widening
- no removal of old route metadata
- no perf keeper claim

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_map_has_directabi_min_v1.mir.json \
  --out /tmp/p72_lowering_plan_map_has_directabi.o
NYASH_LLVM_ROUTE_TRACE=1 bash tools/smokes/v2/profiles/integration/phase29ck_boundary/map/phase29ck_boundary_pure_map_has_no_metadata_min.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
