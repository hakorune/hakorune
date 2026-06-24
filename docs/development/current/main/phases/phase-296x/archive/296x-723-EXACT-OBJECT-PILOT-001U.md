---
Status: Landed
Date: 2026-06-15
Task: EXACT-OBJECT-PILOT-001U
Scope: Enable the first guarded exact-object pilot only after state seam and
  route wiring are green.
Related:
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-722-EXACT-OBJECT-FLATTENED-NESTED-FIELD-ROUTE-WIRING-001.md
  - src/llvm_py/instructions/flattened_nested_fields.py
---

# EXACT-OBJECT-PILOT-001U

## Purpose

The flattened nested field state seam and passive route wiring are now present.
This row is the first row allowed to enable the guarded exact-object pilot for
the selected closed-world front.

Enablement remains narrow:

```text
target_front=object_lifecycle_body
nested_owner=HakoAllocObjectLifecycleFacade.alignment_result
nested_object=HakoAllocObjectLifecycleAlignmentResult
representation_choice=flatten_nested_fields
```

## Required Output

```text
output_contract=hako-exact-object-pilot-u-v0
source_evidence=296x-722
target_front=object_lifecycle_body
nested_owner=HakoAllocObjectLifecycleFacade.alignment_result
nested_object=HakoAllocObjectLifecycleAlignmentResult
state_sharing_seam_defined=1
route_wiring_ready=1
backend_lowering_enabled=<0|1>
object_storage_plan_execution_enabled=<0|1>
pilot_exact_object_enabled=<0|1>
mirbuilder_object_management_enabled=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
fallback_to_generic_box_supported=1
selected_next=EXACT-OBJECT-PILOT-MEASUREMENT-001
summary=<ok|blocked>
```

## Result

```text
output_contract=hako-exact-object-pilot-u-v0
source_evidence=296x-722
target_front=object_lifecycle_body
nested_owner=HakoAllocObjectLifecycleFacade.alignment_result
nested_object=HakoAllocObjectLifecycleAlignmentResult
state_sharing_seam_defined=1
route_wiring_ready=1
field_access_flattened_nested_route_enabled=1
method_call_flattened_nested_route_enabled=1
backend_lowering_enabled=1
object_storage_plan_execution_enabled=1
pilot_exact_object_enabled=1
mirbuilder_object_management_enabled=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
fallback_to_generic_box_supported=1
selected_next=EXACT-OBJECT-PILOT-MEASUREMENT-001
summary=ok
```

Evidence:

```text
tool=tools/allocator/hako_exact_object_pilot_u_enablement.py
report=/tmp/hakorune_723_exact_object_pilot_u_enablement.report
```

## Stop Line

```text
do not change source .hako
do not change MIRBuilder behavior
do not rewrite MIR
do not add benchmark/helper-name branches
do not change product default NyRT
do not generalize to global Arc retirement
```
