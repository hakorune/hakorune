---
Status: Landed
Date: 2026-06-15
Task: EXACT-OBJECT-FLATTENED-NESTED-FIELD-SHADOW-001
Scope: Produce a shadow rewrite plan for
  `HakoAllocObjectLifecycleFacade.alignment_result` flattened nested fields
  without changing execution.
Related:
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-716-EXACT-OBJECT-FLATTENED-NESTED-FIELD-LAYOUT-SSOT-001.md
  - src/object_storage_plan.rs
---

# EXACT-OBJECT-FLATTENED-NESTED-FIELD-SHADOW-001

## Purpose

Convert the passive 716 layout into a shadow rewrite inventory.  The row should
show which `Facade.alignment_result` field get/set and nested method call sites
could be rewritten to flattened primitive fields, but it must not alter
backend lowering.

## Required Output

```text
output_contract=hako-exact-object-flattened-nested-field-shadow-v0
source_evidence=296x-716
target_front=object_lifecycle_body
nested_owner=HakoAllocObjectLifecycleFacade.alignment_result
nested_object=HakoAllocObjectLifecycleAlignmentResult
representation_choice=flatten_nested_fields
flattened_nested_field_count=4
rewritten_get_candidate_count=<n>
rewritten_set_candidate_count=<n>
rewritten_method_candidate_count=<n>
fallback_reason_count=<n>
object_storage_plan_execution_enabled=0
backend_lowering_enabled=0
mirbuilder_object_management_enabled=0
mirbuilder_special_case_count=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
fallback_to_generic_box_supported=1
selected_next=EXACT-OBJECT-FLATTENED-NESTED-FIELD-BACKEND-SEAM-001
summary=<ok|blocked>
```

## Stop Line

```text
do not change source .hako
do not change MIRBuilder behavior
do not add backend lowering behavior
do not rewrite MIR
do not add benchmark/helper-name branches
do not claim product default speedup
```

## Result

```text
output_contract=hako-exact-object-flattened-nested-field-shadow-v0
source_evidence=296x-716
target_front=object_lifecycle_body
nested_owner=HakoAllocObjectLifecycleFacade.alignment_result
nested_object=HakoAllocObjectLifecycleAlignmentResult
representation_choice=flatten_nested_fields
flattened_nested_field_count=4
rewritten_get_candidate_count=7
rewritten_set_candidate_count=1
rewritten_method_candidate_count=7
read_method_candidate_count=4
write_method_candidate_count=3
fallback_reason_count=0
object_storage_plan_execution_enabled=0
backend_lowering_enabled=0
mirbuilder_object_management_enabled=0
mirbuilder_special_case_count=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
fallback_to_generic_box_supported=1
selected_next=EXACT-OBJECT-FLATTENED-NESTED-FIELD-BACKEND-SEAM-001
summary=ok
```

Evidence:

```text
tool=tools/allocator/hako_exact_object_flattened_nested_field_shadow.py
report=/tmp/hakorune_717_exact_object_flattened_nested_field_shadow.report
source_mir_json=/tmp/hakorune_709_post_array_len_owner.1781488363.report.artifacts.d/app.mir.json
```

Decision:

```text
next_task=EXACT-OBJECT-FLATTENED-NESTED-FIELD-BACKEND-SEAM-001
```
