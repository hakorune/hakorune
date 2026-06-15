---
Status: Landed
Date: 2026-06-15
Task: EXACT-OBJECT-PILOT-001R
Scope: Retry the first exact-object pilot using the nested publication plan for
  `HakoAllocObjectLifecycleFacade.alignment_result`.
Related:
  - docs/development/current/main/phases/phase-296x/296x-714-EXACT-OBJECT-NESTED-PUBLICATION-PLAN-001.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - src/object_storage_plan.rs
---

# EXACT-OBJECT-PILOT-001R

## Purpose

Open the first exact-object pilot only after proving that
`HakoAllocObjectLifecycleFacade.alignment_result` does not escape as a handle and
can be represented as flattened nested fields.

## Decision Boundary

The selected nested representation is a plan/backend seam, not a MIRBuilder
truth.  This row may retry the pilot only if an explicit flattened-nested-field
consumer already exists.  If the backend cannot consume
`representation_choice=flatten_nested_fields` without benchmark/helper-name
branches, this row must close as blocked and select the layout seam task.

```text
MIRBuilder:
  source semantics only
  no object representation decision

ObjectStoragePlan:
  owns flattened nested field representation
  owns GenericBox fallback

Backend / exact-AOT:
  consumes the plan
  no source-name / benchmark-name / helper-name special cases
```

## Required Output

```text
output_contract=hako-exact-object-pilot-r-v0
source_evidence=296x-714
target_front=object_lifecycle_body
nested_owner=HakoAllocObjectLifecycleFacade.alignment_result
nested_object=HakoAllocObjectLifecycleAlignmentResult
representation_choice=flatten_nested_fields
object_storage_plan_execution_enabled=<0|1>
pilot_exact_object_enabled=<0|1>
flattened_nested_field_count=<n>
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
do not remove HostHandle globally
do not add benchmark/helper-name branches
do not claim product default speedup
```

## If Blocked

Select the next row instead of local-patching backend lowering:

```text
selected_next=EXACT-OBJECT-FLATTENED-NESTED-FIELD-LAYOUT-SSOT-001
reason=no_explicit_flattened_nested_field_consumer
```

Planned follow-up order:

```text
716 EXACT-OBJECT-FLATTENED-NESTED-FIELD-LAYOUT-SSOT-001
717 EXACT-OBJECT-FLATTENED-NESTED-FIELD-SHADOW-001
718 EXACT-OBJECT-FLATTENED-NESTED-FIELD-BACKEND-SEAM-001
719 EXACT-OBJECT-PILOT-001S
```

## Result

```text
output_contract=hako-exact-object-pilot-r-v0
source_evidence=296x-714
target_front=object_lifecycle_body
nested_owner=HakoAllocObjectLifecycleFacade.alignment_result
nested_object=HakoAllocObjectLifecycleAlignmentResult
representation_choice=flatten_nested_fields
object_storage_plan_execution_enabled=0
pilot_exact_object_enabled=0
flattened_nested_field_count=4
nested_receiver_call_count=7
backend_flattened_nested_field_consumer=0
existing_known_receiver_direct_call_requires_handle=1
local_aggregate_published_nested_consumer=0
mirbuilder_object_management_enabled=0
mirbuilder_special_case_count=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
fallback_to_generic_box_supported=1
selected_next=EXACT-OBJECT-FLATTENED-NESTED-FIELD-LAYOUT-SSOT-001
summary=blocked
```

Evidence:

```text
tool=tools/allocator/hako_exact_object_pilot_retry_preflight.py
report=/tmp/hakorune_715_exact_object_pilot_retry_preflight.report
source_mir_json=/tmp/hakorune_709_post_array_len_owner.1781488363.report.artifacts.d/app.mir.json
```

Decision:

```text
implementation_allowed=0
reason=no_explicit_flattened_nested_field_consumer
next_task=EXACT-OBJECT-FLATTENED-NESTED-FIELD-LAYOUT-SSOT-001
```
