---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SURFACE-001
Scope: Define the report surface and open gate for
  ArrayReceiverResidenceInput before any producer-input implementation.
Related:
  - docs/development/current/main/phases/phase-296x/296x-783-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/296x-782-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INVENTORY-001.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SURFACE-001

## Purpose

296x-783 selected `ArrayReceiverResidenceInput` as the missing
representation input for a later Array receiver residence fact producer.
This row defines the concrete report fields and open gate for that input.

It does not implement the input, the producer, or backend direct handle
bypass.

## Surface

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-residence-producer-input-surface-v0
source_evidence=296x-783,296x-782,array-repr-ssot,object-storage-plan-boundary-ssot,direct-array-access-plan
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

array_receiver_residence_input_surface_defined=1
input_owner=RepresentationPlanner|ObjectStoragePlan|ArrayRepr|none
input_output=ArrayReceiverResidenceInput|none
input_source_routeplan_available=<0|1>
input_source_direct_array_access_plan_available=<0|1>
input_source_object_storage_plan_available=<0|1>
input_source_array_repr_available=<0|1>
input_source_escape_facts_available=<0|1>
input_can_reference_direct_array_access_plan=1
input_is_direct_array_access_plan_only=0
input_supports_length_receiver_residence=<0|1>
input_preserves_public_arraybox_fallback=1
input_public_handle_reinterpretation=0
input_backend_raw_layout_inference=0
input_helper_name_inference=0
input_mirbuilder_owner=0
input_materialization_route_required=1

input_candidate_count=<n>
input_eligible_count=<n>
input_rejected_count=<n>
selected_input_candidate_count=<n>
selected_input_candidate_confidence=low|medium|high
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

A later input implementation row may only open if inventory proves all of:

```text
input_owner=RepresentationPlanner|ObjectStoragePlan|ArrayRepr
input_output=ArrayReceiverResidenceInput
input_source_routeplan_available=1
input_source_escape_facts_available=1
input_can_reference_direct_array_access_plan=1
input_is_direct_array_access_plan_only=0
input_supports_length_receiver_residence=1
input_preserves_public_arraybox_fallback=1
input_public_handle_reinterpretation=0
input_backend_raw_layout_inference=0
input_helper_name_inference=0
input_mirbuilder_owner=0
input_materialization_route_required=1
input_eligible_count>=1
selected_input_candidate_confidence=high
```

This gate still does not permit backend direct handle bypass. It only permits
building the producer input for a later producer inventory/implementation row.

## Stop Line

```text
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
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-INVENTORY-001:
  fill the ArrayReceiverResidenceInput fields from current route,
  DirectArrayAccessPlan, ObjectStoragePlan, ArrayRepr, and escape evidence.
  Keep implementation disabled unless a high-confidence input candidate exists.
```
