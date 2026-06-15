---
Status: Active
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
