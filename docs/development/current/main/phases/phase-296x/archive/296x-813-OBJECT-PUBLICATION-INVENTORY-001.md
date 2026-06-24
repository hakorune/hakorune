---
Status: Landed
Date: 2026-06-16
Task: OBJECT-PUBLICATION-INVENTORY-001
Scope: Inventory conservative publication sites in the object-lifecycle body.
Related:
  - docs/development/current/main/phases/phase-296x/296x-812-OBJECT-PLAN-LOCAL-FIRST-000.md
  - tools/allocator/hako_object_publication_inventory.py
  - lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
---

# OBJECT-PUBLICATION-INVENTORY-001

## Purpose

Apply the local-first object model to the target body as a report-only source
inventory.

This row does not lower anything. It asks:

```text
Which local object candidates exist before publication?
Where must they become public / generic?
```

## Report

```text
output_contract=hako-object-publication-inventory-v0
source_evidence=296x-812,296x-811
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_file=lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
inventory_kind=source_body_conservative
new_box_count=0
local_binding_count=9
local_object_candidate_count=1
preexisting_published_field_alias_count=2
publication_site_count=2
publication_reason_host_handle_required_count=2
publication_reason_plugin_or_extern_count=0
publication_reason_dynamic_array_or_map_count=0
publication_reason_task_future_channel_boundary_count=0
publication_reason_return_as_dynamic_box_count=0
publication_reason_unknown_count=0
record_last_alloc_page_call_count=2
page_local_candidate_count=1
page_publication_site_count=2
pre_publication_page_direct_call_count=3
array_length_direct_candidate_count=0
array_length_direct_candidate_reason=not_in_facade_body
unknown_publication_forces_generic_fallback=1
object_plan_execution_enabled=0
backend_consumes_object_plan=0
product_default_changed=0
summary=ok
```

## Interpretation

The target facade body has one conservative local object candidate:

```text
page
```

It is used locally before publication:

```text
page.acquire_usize(size)
page.reuse()
```

It becomes public through:

```text
me.recordLastAllocPage(..., page)
```

The facade body itself does not contain the Array length direct candidate. That
candidate lives below this surface, so the next shadow row must not claim an
Array length pilot from this inventory alone.

## Stop Line

```text
do not lower page direct calls from this row
do not implement local object storage from this row
do not implement array length direct route from this row
do not infer array length direct candidacy from helper names
do not let backend consume ObjectPlan from this row
do not change product default runtime behavior
```

## Handoff

```text
next_task=LOCAL-OBJECT-SHADOW-001
```
