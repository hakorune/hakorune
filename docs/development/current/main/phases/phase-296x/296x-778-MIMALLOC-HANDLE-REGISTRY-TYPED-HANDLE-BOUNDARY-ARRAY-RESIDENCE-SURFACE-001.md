---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-SURFACE-001
Scope: Define concrete Array receiver residence proof report fields before any
  backend direct handle bypass or storage inventory implementation.
Related:
  - docs/development/current/main/phases/phase-296x/296x-777-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/296x-776-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-INVENTORY-001.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-SURFACE-001

## Purpose

296x-777 selected Array receiver residence proof via ArrayRepr /
ObjectStoragePlan evidence. This row defines the concrete report surface a
later inventory row must fill.

This row does not implement a backend direct handle bypass.

## Decision

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-residence-surface-v0
source_evidence=296x-777,296x-776,array-repr-ssot,object-storage-plan-boundary-ssot
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h
array_receiver_residence_surface_defined=1
array_receiver_route_kind=array_slot_len
array_receiver_residence_owner=ObjectStoragePlan|ArrayRepr|none
array_receiver_residence=direct_array|exact_native_struct|scalarized|public_arraybox_fallback|none
array_receiver_direct_facts_source=DirectArrayAccessPlan|ArrayRepr|ObjectStoragePlan|none
array_receiver_direct_facts_proven=<0|1>
array_receiver_materialization_route_known=<0|1>
array_receiver_materialization_route=public_arraybox_fallback|snapshot|none
array_receiver_public_handle_reinterpreted=0
array_receiver_backend_raw_layout_inference=0
array_receiver_route_proof_as_storage_proof=0
array_receiver_host_handle_publication_before_read=<0|1>
array_receiver_fallback_public_arraybox=1
backend_direct_handle_bypass_enabled=0
implementation_allowed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
mirbuilder_object_management_enabled=0
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-INVENTORY-001
summary=ok
```

## Inventory Report Fields

The next inventory row must emit:

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-residence-inventory-v0
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

array_receiver_residence_surface_defined=1
array_receiver_route_kind=array_slot_len
array_receiver_box_name=ArrayBox
array_receiver_residence_owner=ObjectStoragePlan|ArrayRepr|none
array_receiver_residence=direct_array|exact_native_struct|scalarized|public_arraybox_fallback|none
array_receiver_direct_facts_source=DirectArrayAccessPlan|ArrayRepr|ObjectStoragePlan|none
array_receiver_direct_facts_proven=<0|1>
array_receiver_materialization_route_known=<0|1>
array_receiver_materialization_route=public_arraybox_fallback|snapshot|none
array_receiver_public_handle_reinterpreted=0
array_receiver_backend_raw_layout_inference=0
array_receiver_route_proof_as_storage_proof=0
array_receiver_host_handle_publication_before_read=<0|1>
array_receiver_fallback_public_arraybox=1

residence_candidate_count=<n>
residence_eligible_count=<n>
residence_rejected_count=<n>
selected_residence_candidate_count=<n>
selected_residence_candidate_confidence=low|medium|high
selected_blocker=<blocker|none>

backend_direct_handle_bypass_enabled=0
implementation_allowed=0
product_default_changed=0
summary=ok
```

## Open Gate

A later implementation row may only open if the residence inventory proves all
of:

```text
array_receiver_route_kind=array_slot_len
array_receiver_residence_owner=ObjectStoragePlan|ArrayRepr
array_receiver_residence=direct_array|exact_native_struct|scalarized
array_receiver_direct_facts_source=DirectArrayAccessPlan|ArrayRepr|ObjectStoragePlan
array_receiver_direct_facts_proven=1
array_receiver_materialization_route_known=1
array_receiver_public_handle_reinterpreted=0
array_receiver_backend_raw_layout_inference=0
array_receiver_route_proof_as_storage_proof=0
array_receiver_host_handle_publication_before_read=0
array_receiver_fallback_public_arraybox=1
residence_eligible_count>=1
selected_residence_candidate_confidence=high
```

If any field fails, the hot route remains on the generic HostHandle path.

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
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-INVENTORY-001:
  fill the residence report fields from current ArrayRepr / ObjectStoragePlan
  evidence, keep implementation disabled, and select no implementation unless
  a high-confidence direct residence candidate exists
```
