---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-SURFACE-001
Scope: Define the report surface and open gate for
  ArrayReceiverRepresentationSource.
Related:
  - docs/development/current/main/phases/phase-296x/296x-792-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/296x-791-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-CONSTRUCTION-INVENTORY-001.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-SURFACE-001

## Purpose

296x-792 selected `ArrayReceiverRepresentationSource` as the upstream
representation source consumed by `ArrayReceiverResidenceSourceConstructor`.

This row fixes the concrete report surface and the only open gate for a later
source implementation. It does not implement the source.

## Report Surface

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-surface-v0
source_evidence=296x-792,296x-791,array-repr-ssot,object-storage-plan-boundary-ssot,direct-array-access-plan
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

array_receiver_representation_source_surface_defined=1
representation_source_owner=RepresentationPlanner|ObjectStoragePlan|ArrayRepr|none
representation_source_output=ArrayReceiverRepresentationSource|none
representation_source_scope=receiver_site_before_length_read
representation_source_consumed_by=ArrayReceiverResidenceSourceConstructor
representation_source_route_kind=array_slot_len
representation_source_receiver_box_name=ArrayBox
representation_source_array_repr=DirectI64|PublicArrayBoxFallback|none
representation_source_object_storage_plan_ref=<id|none>
representation_source_direct_array_access_plan_ref=<id|none>
representation_source_materialization_route=public_arraybox_fallback|snapshot|none
representation_source_confidence=low|medium|high
representation_source_may_provide_array_repr=1
representation_source_may_provide_object_storage_plan=1
representation_source_may_reference_direct_array_access_plan=1
representation_source_is_direct_array_access_plan_only=0
representation_source_preserves_public_arraybox_fallback=1
representation_source_includes_materialization_route=<0|1>
representation_source_public_handle_reinterpretation=0
representation_source_backend_raw_layout_inference=0
representation_source_helper_name_inference=0
representation_source_mirbuilder_owner=0

representation_candidate_count=<n>
representation_eligible_count=<n>
representation_rejected_count=<n>
selected_representation_candidate_count=<n>
selected_representation_candidate_confidence=low|medium|high
selected_blocker=<blocker|none>

backend_direct_handle_bypass_enabled=0
implementation_allowed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
mirbuilder_object_management_enabled=0
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-INVENTORY-001
summary=ok
```

## Open Gate

A later implementation row may construct `ArrayReceiverRepresentationSource`
only when an inventory proves all of the following:

```text
representation_source_owner=RepresentationPlanner|ObjectStoragePlan|ArrayRepr
representation_source_output=ArrayReceiverRepresentationSource
representation_source_scope=receiver_site_before_length_read
representation_source_consumed_by=ArrayReceiverResidenceSourceConstructor
representation_source_route_kind=array_slot_len
representation_source_receiver_box_name=ArrayBox
representation_source_may_provide_array_repr=1
representation_source_may_provide_object_storage_plan=1
representation_source_may_reference_direct_array_access_plan=1
representation_source_is_direct_array_access_plan_only=0
representation_source_preserves_public_arraybox_fallback=1
representation_source_includes_materialization_route=1
representation_source_public_handle_reinterpretation=0
representation_source_backend_raw_layout_inference=0
representation_source_helper_name_inference=0
representation_source_mirbuilder_owner=0
representation_eligible_count>=1
selected_representation_candidate_confidence=high
```

## Surface Semantics

`ArrayReceiverRepresentationSource` is a representation proof surface. It can
carry fallback evidence or direct evidence, but it does not by itself authorize
backend direct handle bypass.

```text
PublicArrayBoxFallback:
  valid representation source
  proves fallback/materialized public ArrayBox residence only
  does not prove direct storage

DirectI64:
  may prove direct array representation when carried by ArrayRepr / ObjectStoragePlan

ObjectStoragePlan:
  may prove exact object/aggregate storage when the receiver is local and closed

DirectArrayAccessPlan:
  optional supporting evidence only
  never sufficient as the only representation source
```

## Stop Line

```text
do not implement ArrayReceiverRepresentationSource from this row
do not implement ArrayReceiverResidenceSourceConstructor from this row
do not implement ArrayReceiverResidenceInputSource from this row
do not implement ArrayReceiverResidenceInput from this row
do not implement ArrayReceiverResidenceFact producer from this row
do not implement backend direct handle bypass from this row
do not reinterpret public ArrayBox HostHandle as direct storage
do not infer Rust ArrayBox layout in backend
do not infer representation from helper name
do not move Box/Object management into MIRBuilder
do not change nyash.array.birth_h public semantics
do not retire HostHandle globally
do not retire Arc globally
```

## Next

```text
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-INVENTORY-001:
  fill the representation source surface for the hot Array receiver and select
  whether a high-confidence source candidate exists.
```
