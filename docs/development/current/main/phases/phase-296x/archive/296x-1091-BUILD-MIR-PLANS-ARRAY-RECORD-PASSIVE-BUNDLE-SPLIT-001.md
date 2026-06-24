Status: Done
Date: 2026-06-18
Scope: move ArrayRecord passive metadata bundle into hakorune-mir-plans
Related:
  - docs/development/current/main/phases/phase-296x/296x-1090-BUILD-MIR-PLANS-ARRAY-RECORD-PASSIVE-BUNDLE-SELECTION-001.md
  - crates/hakorune_mir_plans/src/array_record_plan.rs
  - src/mir/function/object_metadata.rs

# BUILD-MIR-PLANS-ARRAY-RECORD-PASSIVE-BUNDLE-SPLIT-001

## Purpose

Move record layout and ArrayRecord passive plan rows into `hakorune-mir-plans`,
while preserving the existing `crate::mir::function::*` compatibility import
surface.

## Change

```text
new_owner=crates/hakorune_mir_plans/src/array_record_plan.rs
moved_rows=RecordLayoutPlan,ArrayRecordStoragePlan,ArrayRecordAutoUseEligibilityPlan,ArrayRecordMaterializationBoundaryPlan,ArrayRecordPackedAutoUsePilotPlan,SourcePackedArrayAutoUsePilotPlan,SourcePackedArrayDirectReadConsumptionPlan,HakoAllocAlignedSmallPackedStorePilotPlan,HakoAllocHugePagePackedStorePilotPlan
main_crate_compat_reexport=crate::mir::function::*
behavior_changed=0
```

The main crate keeps all active producer/classifier logic. The new crate module
contains only passive data rows and small tests that pin the metadata surface.

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
output_contract=build-mir-plans-array-record-passive-bundle-split-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
main_crate_import_path_preserved=1
producer_logic_moved=0
new_backend_lowering_enabled=0
new_runtime_route_enabled=0
new_large_file_created=0

summary=ok
```

## Next

```text
next_task=BUILD-MIR-PLANS-NEXT-PASSIVE-FAMILY-SELECTION-001
```
