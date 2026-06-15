---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-SURFACE-001
Scope: Define concrete report fields for Array receiver storage proof before
  any closed-world handle bypass implementation.
Related:
  - docs/development/current/main/phases/phase-296x/296x-774-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-DESIGN-001.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-SURFACE-001

## Purpose

296x-774 selected Array receiver storage proof via ObjectStoragePlan /
ArrayRepr. This row defines the concrete report surface a later inventory row
must fill.

This row does not implement direct handle bypass.

## Decision

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-storage-proof-surface-v0
source_evidence=296x-774,array-repr-ssot,object-storage-plan-boundary-ssot
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h
array_receiver_storage_proof_surface_defined=1
array_receiver_storage_proof_defined=1
array_receiver_storage_owner=ObjectStoragePlan|ArrayRepr
array_receiver_storage_residence=direct_array|exact_native_struct|scalarized|none
array_receiver_public_handle_reinterpreted=0
array_receiver_host_handle_publication_before_read=<0|1>
array_receiver_fallback_public_arraybox=1
array_receiver_backend_raw_layout_inference=0
array_receiver_direct_facts_required=1
array_receiver_materialization_route_required=1
backend_direct_handle_bypass_enabled=0
implementation_allowed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
mirbuilder_object_management_enabled=0
public_arraybox_handle_reinterpretation_allowed=0
raw_arraybox_layout_backend_truth=0
fallback_to_public_arraybox_host_handle_required=1
fallback_to_generic_host_handle_required=1
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-INVENTORY-001
summary=ok
```

## Inventory Report Fields

The next inventory row must emit:

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-storage-proof-inventory-v0
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

array_receiver_storage_proof_surface_defined=1
array_receiver_route_kind=array_slot_len
array_receiver_box_name=ArrayBox
array_receiver_storage_owner=ObjectStoragePlan|ArrayRepr|none
array_receiver_storage_residence=direct_array|exact_native_struct|scalarized|none
array_receiver_direct_facts_proven=<0|1>
array_receiver_materialization_route_known=<0|1>
array_receiver_public_handle_reinterpreted=0
array_receiver_backend_raw_layout_inference=0
array_receiver_host_handle_publication_before_read=<0|1>
array_receiver_fallback_public_arraybox=1

storage_candidate_count=<n>
storage_eligible_count=<n>
storage_rejected_count=<n>
selected_storage_candidate_count=<n>
selected_storage_candidate_confidence=low|medium|high

backend_direct_handle_bypass_enabled=0
implementation_allowed=0
product_default_changed=0
summary=ok
```

## Open Gate

A later implementation row may only open if the storage inventory proves all
of:

```text
array_receiver_storage_owner=ObjectStoragePlan|ArrayRepr
array_receiver_storage_residence=direct_array|exact_native_struct|scalarized
array_receiver_direct_facts_proven=1
array_receiver_materialization_route_known=1
array_receiver_public_handle_reinterpreted=0
array_receiver_backend_raw_layout_inference=0
array_receiver_host_handle_publication_before_read=0
array_receiver_fallback_public_arraybox=1
storage_eligible_count>=1
selected_storage_candidate_confidence=high
```

If any field fails, the row must reject the bypass and keep the generic
HostHandle route.

## Stop Line

```text
do not implement backend direct handle bypass from this row
do not reinterpret public ArrayBox HostHandle as direct storage
do not expose Rust ArrayBox internal layout as backend truth
do not treat route proof alone as storage proof
do not edit nyash_array_length_h from this row
do not move Box/Object management into MIRBuilder
do not retire HostHandle globally
do not retire Arc globally
```

## Next

```text
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-INVENTORY-001:
  fill the storage proof report fields from current ArrayRepr /
  ObjectStoragePlan evidence, keep implementation disabled, and select no
  implementation unless a high-confidence storage candidate exists
```
