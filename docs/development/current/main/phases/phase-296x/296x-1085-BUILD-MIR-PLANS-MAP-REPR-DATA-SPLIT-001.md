Status: Done
Date: 2026-06-18
Scope: move map_repr_plan pure data subset into hakorune-mir-plans
Related:
  - docs/development/current/main/phases/phase-296x/296x-1084-BUILD-MIR-PLANS-THIRD-FAMILY-SELECTION-001.md
  - crates/hakorune_mir_plans/src/map_repr_plan/
  - src/mir/map_repr_plan/plans.rs

# BUILD-MIR-PLANS-MAP-REPR-DATA-SPLIT-001

## Purpose

Move the pure `map_repr_plan` data subset into `hakorune-mir-plans`, while
keeping MIR-scanning refresh logic and `GenericMethodRoute` translation in the
main crate.

## Change

```text
new_owner=crates/hakorune_mir_plans/src/map_repr_plan
main_crate_builder_facade=src/mir/map_repr_plan/plans.rs
moved_data_types=MapReprKind,MapReprPlan,LocalMapStorageRealizationPlan,LocalI64MapDirectStoragePlan,LocalI64MapEntryValueTrackingPlan
kept_main_crate_logic=refresh,candidates,route_translation,value_origin_const_lookup
behavior_changed=0
```

The main crate facade now converts `GenericMethodRoute` and `MirFunction`
evidence into pure plan data. Backend-visible metadata accessors remain on the
same type names.

## Verification

```text
cargo_test_hakorune_mir_plans=green
cargo_check=green
large_file_count=0
```

## Contract

```text
output_contract=build-mir-plans-map-repr-data-split-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
new_backend_lowering_enabled=0
new_runtime_route_enabled=0
main_crate_refresh_owner_preserved=1
new_large_file_created=0

summary=ok
```

## Next

```text
next_task=BUILD-MIR-PLANS-FOURTH-FAMILY-SELECTION-001
```
