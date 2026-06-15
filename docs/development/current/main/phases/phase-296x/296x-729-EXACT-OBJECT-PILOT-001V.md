---
Status: Landed
Date: 2026-06-15
Task: EXACT-OBJECT-PILOT-001V
Scope: Retry the guarded exact-object pilot after the measured ny-llvmc
  boundary route consumes exported ObjectStoragePlan metadata.
Related:
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-728-EXACT-OBJECT-FLATTENED-NESTED-FIELD-BOUNDARY-CONSUMER-001.md
---

# EXACT-OBJECT-PILOT-001V

## Purpose

`EXACT-OBJECT-FLATTENED-NESTED-FIELD-BOUNDARY-CONSUMER-001` connected the
measured `ny-llvmc` boundary C ABI route to the exported ObjectStoragePlan:

```text
boundary_driver_flattened_nested_consumer=1
field_access_lowering_connected=1
nested_method_lowering_connected=1
generated_artifact_reachability_proven=1
```

This row retries the exact-object pilot at the pre-measurement level.  It does
not claim a body-time win.  It only proves that the selected closed-world front
is enabled through the measured boundary route and may be measured next.

## Required Output

```text
output_contract=hako-exact-object-pilot-v-v0
source_evidence=296x-728
target_front=object_lifecycle_body
nested_owner=HakoAllocObjectLifecycleFacade.alignment_result
nested_object=HakoAllocObjectLifecycleAlignmentResult
representation_choice=flattened_nested_fields
boundary_driver_flattened_nested_consumer=1
field_access_lowering_connected=1
nested_method_lowering_connected=1
generated_artifact_reachability_proven=1
backend_lowering_enabled=1
object_storage_plan_execution_enabled=1
pilot_exact_object_enabled=1
mirbuilder_object_management_enabled=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
fallback_to_generic_box_supported=1
selected_next=EXACT-OBJECT-PILOT-MEASUREMENT-002
summary=ok
```

## Result

```text
output_contract=hako-exact-object-pilot-v-v0
source_evidence=296x-728
target_front=object_lifecycle_body
nested_owner=HakoAllocObjectLifecycleFacade.alignment_result
nested_object=HakoAllocObjectLifecycleAlignmentResult
representation_choice=flattened_nested_fields
boundary_driver_flattened_nested_consumer=1
field_access_lowering_connected=1
nested_method_lowering_connected=1
generated_artifact_reachability_proven=1
backend_lowering_enabled=1
object_storage_plan_execution_enabled=1
pilot_exact_object_enabled=1
mirbuilder_object_management_enabled=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
fallback_to_generic_box_supported=1
selected_next=EXACT-OBJECT-PILOT-MEASUREMENT-002
summary=ok
```

Proof:

```text
tools/allocator/hako_exact_object_pilot_v_preflight.py --repo-root .
bash tools/checks/k2_wide_phase296x_exact_object_pilot_v_guard.sh
```

## Task List

```text
1. Run the 728 boundary consumer guard.
2. Run the pilot V preflight adapter.
3. Keep product default, MIRBuilder, and global Arc retirement unchanged.
4. If green, open measurement-002 for product exact-AOT route measurement.
```

## Stop Line

```text
do not claim body-time win
do not claim product NyRT default speedup
do not claim global Arc retirement
do not change MIRBuilder behavior
do not add benchmark/helper-name branches
```
