---
Status: Landed
Date: 2026-06-15
Task: EXACT-OBJECT-FLATTENED-NESTED-FIELD-LAYOUT-SSOT-001
Scope: Define the ObjectStoragePlan layout contract for flattening
  `HakoAllocObjectLifecycleFacade.alignment_result` into nested primitive
  fields before any backend lowering change.
Related:
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-715-EXACT-OBJECT-PILOT-001R.md
  - src/object_storage_plan.rs
---

# EXACT-OBJECT-FLATTENED-NESTED-FIELD-LAYOUT-SSOT-001

## Purpose

Turn the 715 blocker into a narrow layout contract.

`EXACT-OBJECT-PILOT-001R` proved that the selected nested object cannot be
lowered today because the backend has no explicit consumer for
`representation_choice=flatten_nested_fields`.

This row defines the passive layout vocabulary for that consumer without
changing MIRBuilder, product runtime, or backend execution.

## Required Output

```text
output_contract=hako-exact-object-flattened-nested-field-layout-ssot-v0
source_evidence=296x-715
target_front=object_lifecycle_body
nested_owner=HakoAllocObjectLifecycleFacade.alignment_result
nested_object=HakoAllocObjectLifecycleAlignmentResult
representation_choice=flatten_nested_fields
flattened_nested_field_count=4
flattened_field_name[0]=alignment_result.last_requested
flattened_field_name[1]=alignment_result.last_normalized
flattened_field_name[2]=alignment_result.last_reason
flattened_field_name[3]=alignment_result.last_supported
object_storage_plan_execution_enabled=0
backend_lowering_enabled=0
mirbuilder_object_management_enabled=0
mirbuilder_special_case_count=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
fallback_to_generic_box_supported=1
selected_next=EXACT-OBJECT-FLATTENED-NESTED-FIELD-SHADOW-001
summary=<ok|blocked>
```

## Layout Contract

The passive layout must describe the owner field and nested primitive fields:

```text
owner_box=HakoAllocObjectLifecycleFacade
owner_field=alignment_result
nested_box=HakoAllocObjectLifecycleAlignmentResult
nested_field=last_requested:i64
nested_field=last_normalized:i64
nested_field=last_reason:i64
nested_field=last_supported:i64
```

The canonical flattened names are:

```text
alignment_result.last_requested
alignment_result.last_normalized
alignment_result.last_reason
alignment_result.last_supported
```

The names are plan identifiers, not source-visible fields.

## Stop Line

```text
do not change source .hako
do not change MIRBuilder behavior
do not add backend lowering behavior
do not remove HostHandle globally
do not add benchmark/helper-name branches
do not claim product default speedup
```

## Result

```text
output_contract=hako-exact-object-flattened-nested-field-layout-ssot-v0
source_evidence=296x-715
target_front=object_lifecycle_body
nested_owner=HakoAllocObjectLifecycleFacade.alignment_result
nested_object=HakoAllocObjectLifecycleAlignmentResult
representation_choice=flatten_nested_fields
flattened_nested_field_count=4
flattened_field_name[0]=alignment_result.last_requested
flattened_field_name[1]=alignment_result.last_normalized
flattened_field_name[2]=alignment_result.last_reason
flattened_field_name[3]=alignment_result.last_supported
flattened_field_scalar_type[0]=I64
flattened_field_scalar_type[1]=I64
flattened_field_scalar_type[2]=I64
flattened_field_scalar_type[3]=I64
object_storage_plan_execution_enabled=0
backend_lowering_enabled=0
mirbuilder_object_management_enabled=0
mirbuilder_special_case_count=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
fallback_to_generic_box_supported=1
selected_next=EXACT-OBJECT-FLATTENED-NESTED-FIELD-SHADOW-001
summary=ok
```

Evidence:

```text
tool=tools/allocator/hako_exact_object_flattened_nested_field_layout.py
report=/tmp/hakorune_716_exact_object_flattened_nested_field_layout.report
source_mir_json=/tmp/hakorune_709_post_array_len_owner.1781488363.report.artifacts.d/app.mir.json
```

Decision:

```text
next_task=EXACT-OBJECT-FLATTENED-NESTED-FIELD-SHADOW-001
```
