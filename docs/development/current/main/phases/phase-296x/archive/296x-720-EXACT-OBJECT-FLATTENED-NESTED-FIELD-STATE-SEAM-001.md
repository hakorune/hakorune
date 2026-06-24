---
Status: Landed
Date: 2026-06-15
Task: EXACT-OBJECT-FLATTENED-NESTED-FIELD-STATE-SEAM-001
Scope: Design the backend state-sharing seam required before enabling the
  flattened nested field pilot.
Related:
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-719-EXACT-OBJECT-PILOT-001S.md
  - src/llvm_py/instructions/flattened_nested_fields.py
---

# EXACT-OBJECT-FLATTENED-NESTED-FIELD-STATE-SEAM-001

## Purpose

`EXACT-OBJECT-PILOT-001S` proved that a passive backend consumer is not enough.
The pilot needs one state-sharing seam so that:

```text
Facade.birth field_set alignment_result
Facade.* field_get alignment_result
AlignmentResult.* method call
```

all refer to the same flattened nested field state.

This row defines that seam before changing field access or method lowering.

## Required Output

```text
output_contract=hako-exact-object-flattened-nested-field-state-seam-v0
source_evidence=296x-719
target_front=object_lifecycle_body
nested_owner=HakoAllocObjectLifecycleFacade.alignment_result
nested_object=HakoAllocObjectLifecycleAlignmentResult
state_sharing_seam_defined=<0|1>
typed_newbox_preempts_local_aggregate=1
field_access_flattened_nested_route_enabled=0
method_call_flattened_nested_route_enabled=0
backend_lowering_enabled=0
object_storage_plan_execution_enabled=0
pilot_exact_object_enabled=0
mirbuilder_object_management_enabled=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
fallback_to_generic_box_supported=1
selected_next=EXACT-OBJECT-PILOT-001T
summary=<ok|blocked>
```

## Result

```text
output_contract=hako-exact-object-flattened-nested-field-state-seam-v0
source_evidence=296x-719
target_front=object_lifecycle_body
nested_owner=HakoAllocObjectLifecycleFacade.alignment_result
nested_object=HakoAllocObjectLifecycleAlignmentResult
state_sharing_seam_defined=1
typed_newbox_preempts_local_aggregate=1
field_access_flattened_nested_route_enabled=0
method_call_flattened_nested_route_enabled=0
backend_lowering_enabled=0
object_storage_plan_execution_enabled=0
pilot_exact_object_enabled=0
mirbuilder_object_management_enabled=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
fallback_to_generic_box_supported=1
selected_next=EXACT-OBJECT-PILOT-001T
summary=ok
```

Evidence:

```text
tool=tools/allocator/hako_exact_object_flattened_nested_field_state_seam.py
report=/tmp/hakorune_720_exact_object_flattened_nested_field_state_seam.report
```

## Stop Line

```text
do not change source .hako
do not change MIRBuilder behavior
do not rewrite MIR
do not enable lowering before state sharing is explicit
do not add benchmark/helper-name branches
do not claim product default speedup
```
