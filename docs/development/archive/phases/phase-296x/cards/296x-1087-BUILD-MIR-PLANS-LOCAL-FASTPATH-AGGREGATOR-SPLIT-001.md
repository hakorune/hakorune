Status: Done
Date: 2026-06-18
Scope: move LocalFastPathFact pure aggregation into hakorune-mir-plans
Related:
  - docs/development/current/main/phases/phase-296x/296x-1086-BUILD-MIR-PLANS-FOURTH-FAMILY-SELECTION-001.md
  - crates/hakorune_mir_plans/src/local_fastpath_fact.rs
  - src/mir/local_fastpath_fact.rs

# BUILD-MIR-PLANS-LOCAL-FASTPATH-AGGREGATOR-SPLIT-001

## Purpose

Move the pure `MapReprPlan -> LocalFastPathFact` aggregation helper into
`hakorune-mir-plans`, while preserving the main crate as the owner of
`MirFunction.metadata.local_fastpath_facts` assignment.

## Change

```text
new_owner=crates/hakorune_mir_plans/src/local_fastpath_fact.rs
moved_function=build_local_fastpath_facts_from_map_repr_plans
main_crate_facade=src/mir/local_fastpath_fact.rs
main_crate_keeps=refresh_function_local_fastpath_facts
behavior_changed=0
```

The new crate function is pure: it reads passive `MapReprPlan` values and
emits positive `LocalFastPathFact` values. It does not mutate MIR, enable
backend lowering, or change runtime routes.

## Verification

```text
cargo_test_hakorune_mir_plans=green
cargo_check=green
cargo_build_release_bin_hakorune=green
current_state_pointer_guard=green
large_file_count=0
```

## Contract

```text
output_contract=build-mir-plans-local-fastpath-aggregator-split-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
mirfunction_assignment_owner_preserved=1
new_backend_lowering_enabled=0
new_runtime_route_enabled=0
new_large_file_created=0

summary=ok
```

## Next

```text
next_task=BUILD-MIR-PLANS-FIFTH-FAMILY-SELECTION-001
```
