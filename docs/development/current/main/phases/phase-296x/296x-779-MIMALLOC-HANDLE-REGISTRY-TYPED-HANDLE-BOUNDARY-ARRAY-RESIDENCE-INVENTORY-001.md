---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-INVENTORY-001
Scope: Fill the Array receiver residence proof report from current
  ArrayRepr / ObjectStoragePlan evidence before any backend direct handle bypass.
Related:
  - docs/development/current/main/phases/phase-296x/296x-778-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-SURFACE-001.md
  - docs/development/current/main/phases/phase-296x/296x-777-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/296x-381-DIRECTARRAY-FAMILY-NEXT-ORDER-TASKBOARD.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-INVENTORY-001

## Purpose

296x-778 defined the concrete Array receiver residence proof surface. This row
fills that surface from current evidence.

This row does not implement a backend direct handle bypass.

## Inventory

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-residence-inventory-v0
source_evidence=296x-778,296x-777,array-repr-ssot,object-storage-plan-boundary-ssot,directarray-next-order-taskboard
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

array_receiver_residence_surface_defined=1
array_receiver_route_kind=array_slot_len
array_receiver_box_name=ArrayBox
array_receiver_residence_owner=none
array_receiver_residence=none
array_receiver_direct_facts_source=none
array_receiver_direct_facts_proven=0
array_receiver_materialization_route_known=0
array_receiver_materialization_route=none
array_receiver_public_handle_reinterpreted=0
array_receiver_backend_raw_layout_inference=0
array_receiver_route_proof_as_storage_proof=0
array_receiver_host_handle_publication_before_read=1
array_receiver_fallback_public_arraybox=1

residence_candidate_count=1
residence_eligible_count=0
residence_rejected_count=1
selected_residence_candidate_count=0
selected_residence_candidate_confidence=low
selected_blocker=array_receiver_residence_missing

backend_direct_handle_bypass_enabled=0
implementation_allowed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
mirbuilder_object_management_enabled=0
summary=ok
```

## Reading

The callable route is still known:

```text
array_receiver_route_kind=array_slot_len
target_symbol=nyash_array_length_h
```

But the receiver does not currently carry a selected ArrayRepr /
ObjectStoragePlan residence into this hot site. Existing DirectArray /
ArrayRepr work documents the required bridge, but the current object-lifecycle
front has no proven `DirectI64`, `exact_native_struct`, or `scalarized`
residence before the length read.

The current candidate is therefore rejected.

## Candidate Table

```text
candidate=hot_array_receiver_for_array_slot_len
route_known=1
route_kind=array_slot_len
residence_owner=none
residence=none
direct_facts_source=none
direct_facts_proven=0
materialization_route_known=0
materialization_route=none
host_handle_publication_before_read=1
eligible=0
reject_reason=array_receiver_residence_missing
```

## Decision

```text
selected_decision=reject_backend_direct_handle_bypass_until_array_residence_producer_exists
route_proof_available=1
residence_proof_available=0
array_residence_producer_required=1
arrayrepr_or_object_storage_plan_required=1
public_arraybox_handle_reinterpretation_allowed=0
backend_raw_arraybox_layout_truth=0
route_proof_as_storage_proof_allowed=0
fallback_to_public_arraybox_host_handle_required=1
fallback_to_generic_host_handle_required=1
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-DESIGN-001
```

## Stop Line

```text
do not implement backend direct handle bypass from this row
do not reinterpret public ArrayBox HostHandle as direct storage
do not infer Rust ArrayBox layout in backend
do not treat array_slot_len route proof as receiver storage proof
do not change nyash.array.birth_h public semantics
do not move Box/Object management into MIRBuilder
do not retire HostHandle globally
do not retire Arc globally
```

## Next

```text
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-DESIGN-001:
  design the narrow producer of Array receiver residence evidence for this
  front.  The producer must emit explicit ArrayRepr / ObjectStoragePlan facts
  before the length read, keep public ArrayBox fallback available, and keep
  backend direct handle bypass disabled until a later high-confidence inventory
  proves residence and materialization.
```
