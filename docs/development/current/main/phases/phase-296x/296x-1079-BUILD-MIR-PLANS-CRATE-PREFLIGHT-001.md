Status: Done
Date: 2026-06-18
Scope: hakorune-mir-plans first crate preflight
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1078-BUILD-MIR-CORE-GROWTH-ID-SLICE-001.md

# BUILD-MIR-PLANS-CRATE-PREFLIGHT-001

## Purpose

Select the first passive plan family to move into `hakorune-mir-plans` without
changing MIR building, lowering, backend emission, runtime behavior, or
language acceptance.

## Candidate Audit

```text
candidate=object_storage_plan
status=selected
reason=already_modular_passive_vocabulary
root_facade_exists=1
runtime_dependency=0
builder_dependency=0
backend_emission_dependency=0
tests_are_local_to_family=1
```

Rejected for first split:

```text
local_fastpath_fact=bridge_between_map_repr_and_object_storage
map_repr_plan=depends_on_MirFunction_GenericMethodRoute_ValueDefMap
generic_method_route_plan=too_execution_route_heavy_for_first_split
control_flow_lowering=explicitly_deferred_by_build_crate_split_plan
```

## 800-Line Check

```text
large_file_threshold=800
large_file_count=0
new_large_file_created=0
```

## Contract

```text
output_contract=build-mir-plans-crate-preflight-v0

selected_first_plan_family=object_storage_plan
boxshape_only=1
boxcount_allowed=0
behavior_changed=0
implementation_scope=passive_vocabulary_crate_move

summary=ok
```

## Next

```text
next_task=BUILD-MIR-PLANS-OBJECT-STORAGE-SPLIT-001
```
