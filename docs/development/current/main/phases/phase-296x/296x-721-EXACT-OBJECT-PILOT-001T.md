---
Status: Landed
Date: 2026-06-15
Task: EXACT-OBJECT-PILOT-001T
Scope: Re-run the flattened nested field exact-object pilot preflight after the
  state-sharing seam exists.
Related:
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-720-EXACT-OBJECT-FLATTENED-NESTED-FIELD-STATE-SEAM-001.md
  - src/llvm_py/instructions/flattened_nested_fields.py
---

# EXACT-OBJECT-PILOT-001T

## Purpose

`EXACT-OBJECT-FLATTENED-NESTED-FIELD-STATE-SEAM-001` defined the passive
state-sharing seam for `Facade.alignment_result`.  This row re-runs the guarded
pilot preflight and selects the next narrow route-wiring row if field access or
nested method calls still cannot consume that seam.

## Required Output

```text
output_contract=hako-exact-object-pilot-t-v0
source_evidence=296x-720
target_front=object_lifecycle_body
nested_owner=HakoAllocObjectLifecycleFacade.alignment_result
nested_object=HakoAllocObjectLifecycleAlignmentResult
representation_choice=flatten_nested_fields
state_sharing_seam_defined=1
typed_newbox_preempts_local_aggregate=<0|1>
field_access_flattened_nested_route_enabled=<0|1>
method_call_flattened_nested_route_enabled=<0|1>
backend_lowering_enabled=0
object_storage_plan_execution_enabled=0
pilot_exact_object_enabled=0
mirbuilder_object_management_enabled=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
fallback_to_generic_box_supported=1
selected_next=<task>
summary=<ok|blocked>
```

## Result

```text
output_contract=hako-exact-object-pilot-t-v0
source_evidence=296x-720
target_front=object_lifecycle_body
nested_owner=HakoAllocObjectLifecycleFacade.alignment_result
nested_object=HakoAllocObjectLifecycleAlignmentResult
representation_choice=flatten_nested_fields
state_sharing_seam_defined=1
typed_newbox_preempts_local_aggregate=1
field_access_flattened_nested_route_enabled=0
method_call_flattened_nested_route_enabled=0
route_wiring_ready=0
backend_lowering_enabled=0
object_storage_plan_execution_enabled=0
pilot_exact_object_enabled=0
mirbuilder_object_management_enabled=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
fallback_to_generic_box_supported=1
selected_next=EXACT-OBJECT-FLATTENED-NESTED-FIELD-ROUTE-WIRING-001
summary=blocked
```

Evidence:

```text
tool=tools/allocator/hako_exact_object_pilot_t_preflight.py
report=/tmp/hakorune_721_exact_object_pilot_t_preflight.report
```

## Stop Line

```text
do not change source .hako
do not change MIRBuilder behavior
do not rewrite MIR
do not enable backend lowering in this preflight row
do not add benchmark/helper-name branches
do not claim product default speedup
```
