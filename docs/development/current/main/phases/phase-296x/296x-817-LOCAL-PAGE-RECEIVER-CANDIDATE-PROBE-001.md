---
Status: Landed
Date: 2026-06-16
Task: LOCAL-PAGE-RECEIVER-CANDIDATE-PROBE-001
Scope: Probe the selected page receiver candidate before opening a direct-call guard surface.
Related:
  - docs/development/current/main/phases/phase-296x/296x-816-LOCAL-FIRST-DIRECT-PILOT-SELECTION-001.md
  - tools/allocator/hako_local_page_receiver_candidate_probe.py
---

# LOCAL-PAGE-RECEIVER-CANDIDATE-PROBE-001

## Purpose

Confirm the `page` receiver shape selected by the local-first direct pilot
selection row. This row is report-only. It does not prove backend-consumable
closed-world direct calls and does not open implementation.

## Probe Report

```text
output_contract=hako-local-page-receiver-candidate-probe-v0
source_evidence=296x-816,296x-814,296x-813
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
facade_source_file=lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
queue_source_file=lang/src/hako_alloc/memory/object_lifecycle_page_queue_box.hako
page_source_file=lang/src/hako_alloc/memory/page_box.hako
probe_kind=source_body_conservative

page_local_binding_count=1
page_birth_in_body=0
page_from_queue_selection=1
page_from_queue_selection_count=2
page_select_single_fast_path_assignment_count=1
page_select_page_assignment_count=1
page_selector_return_type_known_count=2
page_type_known=1
page_method_surface_known_count=2

page_acquire_usize_call_count=2
page_reuse_call_count=1
page_pre_publication_call_count=3
page_publication_site_count=2
page_call_after_publication_count=0
page_dynamic_api_required_count=0
page_plugin_or_extern_escape_count=0
page_task_boundary_escape_count=0

page_storage_direct_required=0
page_hosthandle_bypass_required=0
closed_world_direct_call_proof_count=0
routeplan_backend_consumable_proof_count=0
candidate_probe_open=1
guard_surface_required=1
implementation_allowed=0
product_default_changed=0
summary=ok
```

## Interpretation

The probe confirms a Tier-1 direct-call candidate:

```text
page:
  local binding exists
  comes from queue selection
  selector return type is HakoAllocPageModel
  acquire_usize/reuse method surface exists
  three calls occur before page publication in this facade body
```

It also confirms this is not a local-created storage pilot:

```text
page_birth_in_body=0
page_storage_direct_required=0
page_hosthandle_bypass_required=0
```

Therefore the next row may define a guard surface for known receiver direct
calls, but it must not implement direct calls yet.

## Stop Line

```text
do not implement direct call from this probe
do not treat page as body-local new object
do not open storage direct route
do not bypass HostHandle
do not infer backend-consumable RoutePlan proof from source method presence
do not special-case page receiver name
do not special-case acquire_usize or reuse
do not change product default runtime behavior
```

## Next Task

```text
LOCAL-KNOWN-RECEIVER-DIRECT-CALL-GUARD-SURFACE-001
```

The guard surface should require:

```text
receiver is pre-publication
receiver type is known
method surface / route candidate is known
dynamic/plugin/task escape is absent before call
storage_direct_required=0
hosthandle_bypass_enabled=0
```
