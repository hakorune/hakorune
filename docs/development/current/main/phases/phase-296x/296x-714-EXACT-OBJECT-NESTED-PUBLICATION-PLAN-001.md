---
Status: Landed
Date: 2026-06-15
Task: EXACT-OBJECT-NESTED-PUBLICATION-PLAN-001
Scope: Design the nested-object publication plan needed before the first
  exact-object pilot can remove a published handle boundary.
Related:
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-713-EXACT-OBJECT-PILOT-001.md
  - src/object_storage_plan.rs
---

# EXACT-OBJECT-NESTED-PUBLICATION-PLAN-001

## Purpose

`EXACT-OBJECT-PILOT-001` proved that the primitive-only
`HakoAllocObjectLifecycleAlignmentResult` is not a safe immediate stack/native
pilot because it is stored and read through
`HakoAllocObjectLifecycleFacade.alignment_result`.

This row designs the representation needed to make that nested publication
explicit instead of adding backend special cases.

## Required Output

```text
output_contract=hako-exact-object-nested-publication-plan-v0
source_evidence=296x-713
target_front=object_lifecycle_body
nested_owner=HakoAllocObjectLifecycleFacade.alignment_result
nested_object=HakoAllocObjectLifecycleAlignmentResult
publication_boundary_count=8
representation_choice=<flatten_nested_fields|materialized_view_handle|blocked>
mirbuilder_object_management_enabled=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
fallback_to_generic_box_supported=1
summary=<ok|blocked>
```

## Stop Line

```text
do not flatten nested object fields without an explicit ObjectStoragePlan
do not remove HostHandle globally
do not bypass facade field semantics
do not add backend special cases keyed by benchmark/helper names
do not move representation decisions into MIRBuilder
```

## Handoff

```text
next_task=EXACT-OBJECT-PILOT-001R
```

## Result

```text
output_contract=hako-exact-object-nested-publication-plan-v0
source_evidence=296x-713
target_front=object_lifecycle_body
nested_owner=HakoAllocObjectLifecycleFacade.alignment_result
nested_object=HakoAllocObjectLifecycleAlignmentResult
publication_boundary_count=8
facade_nested_field_set_count=1
facade_nested_field_get_count=7
nested_receiver_call_count=7
nested_handle_escape_count=0
representation_choice=flatten_nested_fields
mirbuilder_object_management_enabled=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
fallback_to_generic_box_supported=1
summary=ok
```

Evidence:

```text
tool=tools/allocator/hako_exact_object_nested_publication_plan.py
report=/tmp/hakorune_714_exact_object_nested_publication_plan.report
source_mir_json=/tmp/hakorune_709_post_array_len_owner.1781488363.report.artifacts.d/app.mir.json
```

Decision:

```text
next_task=EXACT-OBJECT-PILOT-001R
representation_choice=flatten_nested_fields
implementation_allowed=1
```
