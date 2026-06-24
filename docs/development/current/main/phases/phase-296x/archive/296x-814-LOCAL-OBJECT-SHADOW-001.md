---
Status: Landed
Date: 2026-06-16
Task: LOCAL-OBJECT-SHADOW-001
Scope: Build a report-only local-first ObjectPlan shadow from publication
  inventory.
Related:
  - docs/development/current/main/phases/phase-296x/296x-813-OBJECT-PUBLICATION-INVENTORY-001.md
  - tools/allocator/hako_local_object_shadow.py
---

# LOCAL-OBJECT-SHADOW-001

## Purpose

Shadow-plan the target body without changing behavior.

This row keeps the local-first lane honest:

```text
page:
  local identity candidate before publication

Array.length:
  no candidate in this facade body
```

## Report

```text
output_contract=hako-local-object-shadow-v0
source_evidence=296x-813,296x-812
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
local_object_candidate_count=1
local_identity_object_candidate_count=1
local_scalar_candidate_count=0
local_struct_candidate_count=0
published_fallback_candidate_count=1
publication_site_count=2
pre_publication_direct_call_count=3
array_length_direct_candidate_count=0
local_direct_array_len_pilot_open=0
shadow_plan_behavior_changed=0
object_plan_execution_enabled=0
backend_consumes_object_plan=0
product_default_changed=0
summary=ok
```

## Interpretation

The local-first model has a useful target here:

```text
page local identity object
```

But this is not the `Array.length` target. The facade body does not contain a
pre-publication Array length candidate, so opening `LOCAL-DIRECT-ARRAY-LEN-PILOT`
as an implementation row from this evidence would be wrong.

## Stop Line

```text
do not implement local page direct lowering from this row
do not implement Array.length direct lowering from this row
do not claim LOCAL-DIRECT-ARRAY-LEN-PILOT is open
do not infer array length candidacy from nyash_array_length_h
do not let backend consume ObjectPlan from this row
do not change product default runtime behavior
```

## Handoff

```text
next_task=LOCAL-DIRECT-ARRAY-LEN-PILOT-001
next_task_status=blocked_by_no_candidate_in_target_body
```
