---
Status: Landed
Date: 2026-06-15
Task: EXACT-OBJECT-PILOT-001S
Scope: Enable the first guarded exact-object pilot for
  `HakoAllocObjectLifecycleFacade.alignment_result` using the disabled backend
  seam proven in 718.
Related:
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-718-EXACT-OBJECT-FLATTENED-NESTED-FIELD-BACKEND-SEAM-001.md
  - src/llvm_py/instructions/flattened_nested_fields.py
---

# EXACT-OBJECT-PILOT-001S

## Purpose

Turn the passive flattened nested field seam into the first guarded exact-object
pilot.  This is the first row that may enable exact-object lowering for the
selected closed-world object-lifecycle front.

## Required Output

```text
output_contract=hako-exact-object-pilot-s-v0
source_evidence=296x-718
target_front=object_lifecycle_body
nested_owner=HakoAllocObjectLifecycleFacade.alignment_result
nested_object=HakoAllocObjectLifecycleAlignmentResult
representation_choice=flatten_nested_fields
backend_flattened_nested_field_consumer=1
backend_lowering_enabled=<0|1>
object_storage_plan_execution_enabled=<0|1>
pilot_exact_object_enabled=<0|1>
mirbuilder_object_management_enabled=0
mirbuilder_special_case_count=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
fallback_to_generic_box_supported=1
summary=<ok|blocked>
```

## Stop Line

```text
do not change source .hako
do not change MIRBuilder behavior
do not rewrite MIR
do not remove HostHandle globally
do not add benchmark/helper-name branches
do not claim product default speedup without measurement
```

## Result

```text
output_contract=hako-exact-object-pilot-s-v0
source_evidence=296x-718
target_front=object_lifecycle_body
nested_owner=HakoAllocObjectLifecycleFacade.alignment_result
nested_object=HakoAllocObjectLifecycleAlignmentResult
representation_choice=flatten_nested_fields
backend_flattened_nested_field_consumer=1
backend_lowering_enabled=0
object_storage_plan_execution_enabled=0
pilot_exact_object_enabled=0
typed_newbox_preempts_local_aggregate=1
field_access_flattened_nested_route_enabled=0
method_call_flattened_nested_route_enabled=0
state_sharing_seam_ready=0
mirbuilder_object_management_enabled=0
mirbuilder_special_case_count=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
fallback_to_generic_box_supported=1
selected_next=EXACT-OBJECT-FLATTENED-NESTED-FIELD-STATE-SEAM-001
summary=blocked
```

Evidence:

```text
tool=tools/allocator/hako_exact_object_pilot_s_preflight.py
report=/tmp/hakorune_719_exact_object_pilot_s_preflight.report
```

Decision:

```text
implementation_allowed=0
reason=state_sharing_seam_missing
next_task=EXACT-OBJECT-FLATTENED-NESTED-FIELD-STATE-SEAM-001
```
