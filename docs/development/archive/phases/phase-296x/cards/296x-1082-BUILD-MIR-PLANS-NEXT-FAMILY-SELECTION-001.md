Status: Done
Date: 2026-06-18
Scope: next passive family selection for hakorune-mir-plans
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1081-BUILD-TIME-BASELINE-MEASURE-001.md

# BUILD-MIR-PLANS-NEXT-FAMILY-SELECTION-001

## Purpose

Select the next passive family for `hakorune-mir-plans` after
`object_storage_plan`.

## Candidate Audit

```text
candidate=aggregate_storage_plan
status=selected
lines=130
root_users=src/lib.rs
depends_on_object_storage_plan=1
depends_on_mir_function=0
depends_on_builder=0
depends_on_backend=0
depends_on_runtime=0
behavior_changed_required=0
```

Rejected for this slice:

```text
local_fastpath_fact=depends_on_MirFunction_and_map_repr_plan_refresh_path
map_repr_plan=depends_on_MirFunction_GenericMethodRoute_ValueDefMap
generic_method_route_plan=route_execution_policy_too_large_for_second_slice
string_or_array_plan_files=larger_semantic_route_families_need_separate_audit
```

## 800-Line Check

```text
large_file_threshold=800
candidate_large_file_count=0
new_large_file_created=0
```

## Contract

```text
output_contract=build-mir-plans-next-family-selection-v0

selected_next_family=aggregate_storage_plan
boxshape_only=1
boxcount_allowed=0
behavior_changed=0
implementation_scope=passive_vocabulary_crate_move

summary=ok
```

## Next

```text
next_task=BUILD-MIR-PLANS-AGGREGATE-STORAGE-SPLIT-001
```
