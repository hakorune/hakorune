---
Status: Landed
Date: 2026-06-15
Task: EXACT-OBJECT-FLATTENED-NESTED-FIELD-ROUTE-WIRING-001
Scope: Wire field access and nested direct method call routes to the passive
  flattened nested field state seam.
Related:
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-721-EXACT-OBJECT-PILOT-001T.md
  - src/llvm_py/instructions/flattened_nested_fields.py
---

# EXACT-OBJECT-FLATTENED-NESTED-FIELD-ROUTE-WIRING-001

## Purpose

`EXACT-OBJECT-PILOT-001T` proved the passive state seam exists, but field access
and nested method-call consumers still do not route through it.

This row wires the consumers to the seam while keeping execution disabled:

```text
Facade.birth field_set alignment_result
Facade.* field_get alignment_result
AlignmentResult.* method call
```

must all resolve through the same ObjectStoragePlan state identity before the
pilot may enable backend lowering.

## Required Output

```text
output_contract=hako-exact-object-flattened-nested-field-route-wiring-v0
source_evidence=296x-721
target_front=object_lifecycle_body
nested_owner=HakoAllocObjectLifecycleFacade.alignment_result
nested_object=HakoAllocObjectLifecycleAlignmentResult
state_sharing_seam_defined=1
field_access_flattened_nested_route_enabled=<0|1>
method_call_flattened_nested_route_enabled=<0|1>
route_wiring_ready=<0|1>
backend_lowering_enabled=0
object_storage_plan_execution_enabled=0
pilot_exact_object_enabled=0
mirbuilder_object_management_enabled=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
fallback_to_generic_box_supported=1
selected_next=EXACT-OBJECT-PILOT-001U
summary=<ok|blocked>
```

## Result

```text
output_contract=hako-exact-object-flattened-nested-field-route-wiring-v0
source_evidence=296x-721
target_front=object_lifecycle_body
nested_owner=HakoAllocObjectLifecycleFacade.alignment_result
nested_object=HakoAllocObjectLifecycleAlignmentResult
state_sharing_seam_defined=1
field_access_flattened_nested_route_enabled=1
method_call_flattened_nested_route_enabled=1
route_wiring_ready=1
backend_lowering_enabled=0
object_storage_plan_execution_enabled=0
pilot_exact_object_enabled=0
mirbuilder_object_management_enabled=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
fallback_to_generic_box_supported=1
selected_next=EXACT-OBJECT-PILOT-001U
summary=ok
```

Evidence:

```text
tool=tools/allocator/hako_exact_object_flattened_nested_field_route_wiring.py
report=/tmp/hakorune_722_exact_object_flattened_nested_field_route_wiring.report
```

## Stop Line

```text
do not change source .hako
do not change MIRBuilder behavior
do not rewrite MIR
do not enable backend lowering in this row
do not add benchmark/helper-name branches
do not claim product default speedup
```
