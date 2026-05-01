---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: lock the next LoweringPlan v0 slice before implementation.
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P70-LOWERING-PLAN-V0-EMIT-AND-CONSUME.md
  - apps/tests/mir_shape_guard/map_has_no_metadata_min_v1.mir.json
---

# P71 LoweringPlan DirectAbi MapHas Lock

## Goal

Add the next LoweringPlan v0 proof without turning ny-llvmc back into a method
classifier.

P70 proved `MapGet` as `ColdRuntime`. P71 selects `MapHas` as the first
`DirectAbi` plan-only proof because the existing CoreMethodContract,
CoreMethodOp, route metadata, helper symbol, and boundary smoke already agree on
the shape.

## Decision

- The implementation slice after this card targets only:
  - `core_op = MapHas`
  - `tier = DirectAbi`
  - `emit_kind = direct_abi_call`
  - `symbol = nyash.map.probe_hi`
  - `route_kind = map_contains_i64`
- The plan-only fixture must omit `generic_method_routes`.
- ny-llvmc should read `metadata.lowering_plan` first and keep legacy
  `generic_method_routes` as migration fallback only.
- This is not a new accepted method surface. It is an ownership move for an
  already-supported shape.

## Non-goals

- do not remove legacy `generic_method_routes`
- do not add a new CoreMethodOp
- do not widen `MapBox.has` beyond the existing `map_contains_i64` DirectAbi row
- do not promote any `ColdRuntime` path to perf proof

## Acceptance For Implementation

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_map_has_directabi_min_v1.mir.json \
  --out /tmp/p71_lowering_plan_map_has_directabi.o
NYASH_LLVM_ROUTE_TRACE=1 bash tools/smokes/v2/profiles/integration/phase29ck_boundary/map/phase29ck_boundary_pure_map_has_no_metadata_min.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
