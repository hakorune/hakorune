---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-INVENTORY-001
Scope: Fill the Array receiver storage proof inventory from current
  ObjectStoragePlan / ArrayRepr evidence before any closed-world handle bypass.
Related:
  - docs/development/current/main/phases/phase-296x/296x-775-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-SURFACE-001.md
  - docs/development/current/main/phases/phase-296x/296x-774-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-DESIGN-001.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-INVENTORY-001

## Purpose

296x-775 defined the storage-proof report surface required before bypassing
the hot `nyash_array_length_h` HostHandle boundary.

This row fills that report from current evidence. It does not implement a
backend direct handle bypass.

## Inventory

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-storage-proof-inventory-v0
source_evidence=296x-775,296x-774,array-repr-ssot,object-storage-plan-boundary-ssot
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

array_receiver_storage_proof_surface_defined=1
array_receiver_route_kind=array_slot_len
array_receiver_box_name=ArrayBox
array_receiver_storage_owner=none
array_receiver_storage_residence=none
array_receiver_direct_facts_proven=0
array_receiver_materialization_route_known=0
array_receiver_public_handle_reinterpreted=0
array_receiver_backend_raw_layout_inference=0
array_receiver_host_handle_publication_before_read=1
array_receiver_fallback_public_arraybox=1

storage_candidate_count=1
storage_eligible_count=0
storage_rejected_count=1
selected_storage_candidate_count=0
selected_storage_candidate_confidence=low
selected_blocker=array_receiver_storage_residence_missing

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

The hot site has a route proof:

```text
route_kind=array_slot_len
box_name=ArrayBox
target_symbol=nyash_array_length_h
```

But the receiver is still a public `ArrayBox` route for this front. Current
evidence does not prove an ArrayRepr / ObjectStoragePlan residence such as
`direct_array`, `exact_native_struct`, or `scalarized` for the receiver before
the length read.

Therefore the current storage candidate is rejected.

## Candidate Table

```text
candidate=hot_array_receiver_for_array_slot_len
route_known=1
route_kind=array_slot_len
storage_owner=none
storage_residence=none
direct_facts_proven=0
materialization_route_known=0
host_handle_publication_before_read=1
eligible=0
reject_reason=array_receiver_storage_residence_missing
```

## Decision

```text
selected_decision=reject_backend_direct_handle_bypass_until_array_residence_exists
route_proof_available=1
storage_proof_available=0
arrayrepr_or_object_storage_plan_required=1
public_arraybox_handle_reinterpretation_allowed=0
raw_arraybox_layout_backend_truth=0
fallback_to_public_arraybox_host_handle_required=1
fallback_to_generic_host_handle_required=1
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-DESIGN-001
```

## Stop Line

```text
do not implement backend direct handle bypass from this row
do not reinterpret public ArrayBox HostHandle as direct storage
do not infer Rust ArrayBox layout in backend
do not treat array_slot_len route proof as receiver storage proof
do not edit nyash_array_length_h from this row
do not move Box/Object management into MIRBuilder
do not retire HostHandle globally
do not retire Arc globally
```

## Next

```text
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-DESIGN-001:
  design the narrow Array receiver residence proof needed for this front.
  The proof must create explicit ObjectStoragePlan / ArrayRepr evidence before
  the length read, keep public ArrayBox fallback available, and keep direct
  backend handle bypass disabled until a later high-confidence inventory row.
```
