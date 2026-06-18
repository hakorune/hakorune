Status: Done
Date: 2026-06-18
Scope: move MIR function fact passive rows into hakorune-mir-plans
Related:
  - docs/development/current/main/phases/phase-296x/296x-1094-BUILD-MIR-PLANS-STAGE1-CLOSEOUT-CANDIDATE-001.md
  - crates/hakorune_mir_plans/src/function_fact_plan.rs
  - src/mir/function/facts.rs

# BUILD-MIR-PLANS-FUNCTION-FACT-PASSIVE-BUNDLE-SPLIT-001

## Purpose

Move the remaining low-risk MIR function fact/plan vocabulary into
`hakorune-mir-plans`, while preserving the existing
`crate::mir::function::*` import surface.

## Change

```text
new_owner=crates/hakorune_mir_plans/src/function_fact_plan.rs
moved_rows=LoopRangeFact,CountingLoopFact,RangeIndexFact,RangeIndexFactOriginKind,DirectArrayExtentFact,DirectArrayExtentProofKind,RegionStabilityFact,RegionStabilityProofKind,SpanBorrowFact,SpanBorrowMutability,SpanElementType,SpanAccessPlan,SpanAccessOp,RequiredFastPathRegion,FastPathObligation
main_crate_compat_reexport=crate::mir::function::*
behavior_changed=0
```

The main crate keeps all producers and refresh logic:

```text
range_index_fact_producer_moved=0
direct_array_extent_fact_producer_moved=0
span_access_plan_producer_moved=0
fastmem_producer_moved=0
```

## Verification

```text
cargo_test_hakorune_mir_plans=green
cargo_check=green
```

## Contract

```text
output_contract=build-mir-plans-function-fact-passive-bundle-split-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
main_crate_import_path_preserved=1
producer_logic_moved=0
refresh_logic_moved=0
new_backend_lowering_enabled=0
new_runtime_route_enabled=0
new_large_file_created=0

summary=ok
```

## Next

```text
next_task=BUILD-MIR-PLANS-STAGE1-CLOSEOUT-001
```
