Status: Done
Date: 2026-06-18
Scope: third passive family selection for hakorune-mir-plans
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1083-BUILD-MIR-PLANS-AGGREGATE-STORAGE-SPLIT-001.md

# BUILD-MIR-PLANS-THIRD-FAMILY-SELECTION-001

## Purpose

Select the next `hakorune-mir-plans` family after `object_storage_plan` and
`aggregate_storage_plan`.

## Candidate Audit

```text
candidate=map_repr_plan_pure_data_subset
status=selected
whole_family_lines=647
pure_data_subset_lines=407
depends_on_mir_function_before_split=1
selected_slice_depends_on_mir_function_after_split=0
selected_slice_depends_on_backend=0
selected_slice_depends_on_runtime=0
```

Rejected for this slice:

```text
map_repr_refresh_logic=depends_on_MirFunction_GenericMethodRoute_ValueDefMap
map_repr_candidate_detection=depends_on_MirInstruction_scan
local_fastpath_fact=depends_on_MirFunction_and_map_repr_refresh_path
generic_method_route_plan=route_execution_policy_too_large_for_third_slice
```

## Decision

Move only the pure data structs and tags:

```text
move=MapReprKind
move=MapReprPlan
move=LocalMapStorageRealizationPlan
move=LocalI64MapDirectStoragePlan
move=LocalI64MapEntryValueTrackingPlan
keep_in_main_crate=map_repr_refresh
keep_in_main_crate=map_repr_candidate_detection
keep_in_main_crate=GenericMethodRoute_to_MapReprPlan_builders
```

## Contract

```text
output_contract=build-mir-plans-third-family-selection-v0

selected_third_family=map_repr_plan_pure_data_subset
boxshape_only=1
boxcount_allowed=0
behavior_changed=0
implementation_scope=pure_data_struct_move_only
new_large_file_created=0

summary=ok
```

## Next

```text
next_task=BUILD-MIR-PLANS-MAP-REPR-DATA-SPLIT-001
```
