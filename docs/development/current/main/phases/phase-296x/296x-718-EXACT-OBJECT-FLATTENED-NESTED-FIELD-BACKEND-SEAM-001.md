---
Status: Landed
Date: 2026-06-15
Task: EXACT-OBJECT-FLATTENED-NESTED-FIELD-BACKEND-SEAM-001
Scope: Add the disabled exact-AOT backend seam that can consume the flattened
  nested field shadow plan.
Related:
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-717-EXACT-OBJECT-FLATTENED-NESTED-FIELD-SHADOW-001.md
  - src/object_storage_plan.rs
---

# EXACT-OBJECT-FLATTENED-NESTED-FIELD-BACKEND-SEAM-001

## Purpose

Add the backend-side seam that can consume flattened nested field plans without
enabling execution yet.  This row should make the next pilot implementation a
guarded plan consumption problem rather than a local backend special case.

## Required Output

```text
output_contract=hako-exact-object-flattened-nested-field-backend-seam-v0
source_evidence=296x-717
target_front=object_lifecycle_body
representation_choice=flatten_nested_fields
flattened_nested_field_count=4
backend_flattened_nested_field_consumer=1
backend_lowering_enabled=0
object_storage_plan_execution_enabled=0
pilot_exact_object_enabled=0
mirbuilder_object_management_enabled=0
mirbuilder_special_case_count=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
fallback_to_generic_box_supported=1
selected_next=EXACT-OBJECT-PILOT-001S
summary=<ok|blocked>
```

## Stop Line

```text
do not enable exact-object execution in this row
do not change source .hako
do not change MIRBuilder behavior
do not rewrite MIR
do not add benchmark/helper-name branches
do not claim product default speedup
```

## Result

```text
output_contract=hako-exact-object-flattened-nested-field-backend-seam-v0
source_evidence=296x-717
target_front=object_lifecycle_body
representation_choice=flatten_nested_fields
flattened_nested_field_count=4
backend_flattened_nested_field_consumer=1
backend_lowering_enabled=0
object_storage_plan_execution_enabled=0
pilot_exact_object_enabled=0
mirbuilder_object_management_enabled=0
mirbuilder_special_case_count=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
fallback_to_generic_box_supported=1
selected_next=EXACT-OBJECT-PILOT-001S
summary=ok
```

Evidence:

```text
tool=tools/allocator/hako_exact_object_flattened_nested_field_backend_seam.py
report=/tmp/hakorune_718_exact_object_flattened_nested_field_backend_seam.report
backend_seam=src/llvm_py/instructions/flattened_nested_fields.py
```

Decision:

```text
next_task=EXACT-OBJECT-PILOT-001S
```
