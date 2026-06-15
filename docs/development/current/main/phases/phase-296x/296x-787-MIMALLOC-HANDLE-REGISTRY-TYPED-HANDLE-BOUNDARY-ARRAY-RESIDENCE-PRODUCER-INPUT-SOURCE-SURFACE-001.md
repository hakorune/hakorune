---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-SURFACE-001
Scope: Define the report surface and open gate for
  ArrayReceiverResidenceInputSource before implementation.
Related:
  - docs/development/current/main/phases/phase-296x/296x-786-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/296x-785-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-INVENTORY-001.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-SURFACE-001

## Purpose

296x-786 selected `ArrayReceiverResidenceInputSource` as the missing
representation-planner source consumed by `ArrayReceiverResidenceInput`.
This row defines the concrete report fields and open gate for that source.

It does not implement the source, the input, the producer, or backend direct
handle bypass.

## Surface

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-residence-producer-input-source-surface-v0
source_evidence=296x-786,296x-785,array-repr-ssot,object-storage-plan-boundary-ssot,direct-array-access-plan
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

array_receiver_residence_input_source_surface_defined=1
input_source_owner=RepresentationPlanner|ArrayReprSourcePlanner|ObjectStoragePlan|none
input_source_output=ArrayReceiverResidenceInputSource|none
input_source_scope=receiver_site_before_length_read
input_source_consumed_by=ArrayReceiverResidenceInput
input_source_route_kind=array_slot_len
input_source_receiver_box_name=ArrayBox
input_source_routeplan_available=<0|1>
input_source_direct_array_access_plan_ref=<id|none>
input_source_object_storage_plan_ref=<id|none>
input_source_array_repr=DirectI64|PublicArrayBoxFallback|none
input_source_escape_facts_ref=<id|none>
input_source_host_handle_publication_before_read=<0|1>
input_source_materialization_route=public_arraybox_fallback|snapshot|none
input_source_confidence=low|medium|high

input_source_may_reference_direct_array_access_plan=1
input_source_is_direct_array_access_plan_only=0
input_source_includes_array_repr_or_object_storage=<0|1>
input_source_includes_escape_publication_evidence=<0|1>
input_source_includes_materialization_route=<0|1>
input_source_preserves_public_arraybox_fallback=1
input_source_public_handle_reinterpretation=0
input_source_backend_raw_layout_inference=0
input_source_helper_name_inference=0
input_source_mirbuilder_owner=0

source_candidate_count=<n>
source_eligible_count=<n>
source_rejected_count=<n>
selected_source_candidate_count=<n>
selected_source_candidate_confidence=low|medium|high
selected_blocker=<blocker|none>

backend_direct_handle_bypass_enabled=0
implementation_allowed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
mirbuilder_object_management_enabled=0
summary=ok
```

## Open Gate

A later input-source implementation row may only open if inventory proves all
of:

```text
input_source_owner=RepresentationPlanner|ArrayReprSourcePlanner|ObjectStoragePlan
input_source_output=ArrayReceiverResidenceInputSource
input_source_scope=receiver_site_before_length_read
input_source_consumed_by=ArrayReceiverResidenceInput
input_source_route_kind=array_slot_len
input_source_receiver_box_name=ArrayBox
input_source_routeplan_available=1
input_source_may_reference_direct_array_access_plan=1
input_source_is_direct_array_access_plan_only=0
input_source_includes_array_repr_or_object_storage=1
input_source_includes_escape_publication_evidence=1
input_source_includes_materialization_route=1
input_source_preserves_public_arraybox_fallback=1
input_source_public_handle_reinterpretation=0
input_source_backend_raw_layout_inference=0
input_source_helper_name_inference=0
input_source_mirbuilder_owner=0
source_eligible_count>=1
selected_source_candidate_confidence=high
```

This gate permits only source construction. It still does not permit producing
`ArrayReceiverResidenceInput`, `ArrayReceiverResidenceFact`, or backend direct
handle bypass.

## Stop Line

```text
do not implement the input source from this row
do not implement the producer input from this row
do not implement the producer from this row
do not implement backend direct handle bypass from this row
do not reinterpret public ArrayBox HostHandle as direct storage
do not infer Rust ArrayBox layout in backend
do not treat DirectArrayAccessPlan alone as receiver residence proof
do not treat array_slot_len route proof as receiver storage proof
do not change nyash.array.birth_h public semantics
do not move Box/Object management into MIRBuilder
do not retire HostHandle globally
do not retire Arc globally
```

## Next

```text
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-INVENTORY-001:
  fill the ArrayReceiverResidenceInputSource fields from current route,
  DirectArrayAccessPlan, ObjectStoragePlan, ArrayRepr, escape/publication, and
  materialization evidence. Keep implementation disabled unless a
  high-confidence source candidate exists.
```
