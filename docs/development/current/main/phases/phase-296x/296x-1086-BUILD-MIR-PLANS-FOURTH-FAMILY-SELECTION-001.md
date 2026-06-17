Status: Done
Date: 2026-06-18
Scope: select the fourth passive family for hakorune-mir-plans split
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - src/mir/local_fastpath_fact.rs
  - src/mir/array_record_storage_plan.rs
  - src/mir/record_layout_plan.rs

# BUILD-MIR-PLANS-FOURTH-FAMILY-SELECTION-001

## Purpose

Choose the next safe `hakorune-mir-plans` split after `map_repr_plan` pure
data moved into the new crate.

## Audit

```text
candidate=local_fastpath_fact
dependency_shape=MapReprPlan + ObjectStoragePlan vocabulary
main_crate_dependency=MirFunction metadata assignment only
decision=select_pure_aggregator_subset

candidate=array_record_storage_plan
dependency_shape=MirModule + MirFunction metadata structs
decision=defer_until_metadata_structs_move

candidate=record_layout_plan
dependency_shape=declared_type_storage + MirModule + UserBoxFieldDecl
decision=defer

candidate=direct_array_extent_fact
dependency_shape=MirFunction + MirInstruction + ValueDefMap
decision=defer_not_passive
```

## Decision

Select `local_fastpath_fact` pure aggregation as the fourth family.

The move is intentionally narrow:

```text
move=MapReprPlan[] -> LocalFastPathFact[] pure aggregator
keep_main_crate=refresh_function_local_fastpath_facts(MirFunction)
behavior_changed=0
```

The main crate remains the owner for final assignment into
`MirFunction.metadata.local_fastpath_facts`. The new crate owns only passive
plan-to-fact aggregation.

## Contract

```text
output_contract=build-mir-plans-fourth-family-selection-v0

selected_family=local_fastpath_fact_pure_aggregator
boxshape_only=1
boxcount_allowed=0
behavior_change_allowed=0
mirfunction_mutation_moved=0
backend_lowering_enabled=0
runtime_route_enabled=0

summary=ok
```

## Next

```text
next_task=BUILD-MIR-PLANS-LOCAL-FASTPATH-AGGREGATOR-SPLIT-001
```
