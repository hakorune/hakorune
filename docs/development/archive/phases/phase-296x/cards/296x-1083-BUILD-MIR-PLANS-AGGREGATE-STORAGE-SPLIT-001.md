Status: Done
Date: 2026-06-18
Scope: move aggregate_storage_plan into hakorune-mir-plans
Related:
  - docs/development/current/main/phases/phase-296x/296x-1082-BUILD-MIR-PLANS-NEXT-FAMILY-SELECTION-001.md
  - crates/hakorune_mir_plans/src/aggregate_storage_plan.rs
  - src/aggregate_storage_plan.rs

# BUILD-MIR-PLANS-AGGREGATE-STORAGE-SPLIT-001

## Purpose

Move the passive AggregateStoragePlan vocabulary into `hakorune-mir-plans`
next to ObjectStoragePlan, while preserving the main-crate
`crate::aggregate_storage_plan` facade.

## Change

```text
family=aggregate_storage_plan
new_owner=crates/hakorune_mir_plans/src/aggregate_storage_plan.rs
main_crate_compat_facade=src/aggregate_storage_plan.rs
behavior_changed=0
```

## Contract

```text
output_contract=build-mir-plans-aggregate-storage-split-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
aggregate_storage_plan_execution_enabled=0
mirbuilder_representation_owner=0
new_large_file_created=0

summary=ok
```

## Next

```text
next_task=BUILD-MIR-PLANS-THIRD-FAMILY-SELECTION-001
```
