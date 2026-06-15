---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-DESIGN-001
Scope: Design the narrow Array receiver residence proof needed before the hot
  Array length HostHandle boundary can be bypassed.
Related:
  - docs/development/current/main/phases/phase-296x/296x-776-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-INVENTORY-001.md
  - docs/development/current/main/phases/phase-296x/296x-775-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-SURFACE-001.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-373-ARRAYBOX-PUBLIC-SEMANTICS-AND-DIRECTARRAY-SPLIT-SSOT.md
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-DESIGN-001

## Purpose

296x-776 proved the current hot `ArrayBox.length` route is known, but rejected
backend direct handle bypass because the receiver has no storage residence
proof:

```text
array_receiver_storage_owner=none
array_receiver_storage_residence=none
array_receiver_direct_facts_proven=0
array_receiver_host_handle_publication_before_read=1
```

This row decides how that residence proof should be created without turning the
public `ArrayBox` handle or Rust runtime layout into backend truth.

## Decision

Create a narrow Array receiver residence proof surface owned by
ObjectStoragePlan / ArrayRepr evidence.

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-residence-design-v0
source_evidence=296x-776,296x-775,296x-373,array-repr-ssot,object-storage-plan-boundary-ssot
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h
selected_design=array_receiver_residence_proof_via_arrayrepr
selected_design_confidence=medium
route_proof_available=1
storage_proof_available=0
array_residence_proof_required=1
array_residence_owner=ObjectStoragePlan|ArrayRepr
array_residence_allowed_values=direct_array|exact_native_struct|scalarized|public_arraybox_fallback
array_direct_facts_source=DirectArrayAccessPlan|ArrayRepr|ObjectStoragePlan
array_materialization_route_required=1
public_arraybox_fallback_required=1
public_arraybox_handle_reinterpretation_allowed=0
backend_raw_arraybox_layout_truth=0
route_proof_as_storage_proof_allowed=0
mirbuilder_object_management_enabled=0
backend_direct_handle_bypass_enabled=0
implementation_allowed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-SURFACE-001
summary=ok
```

## Responsibility Split

```text
RoutePlan:
  proves callable route:
    ArrayBox.length -> array_slot_len

ArrayRepr / ObjectStoragePlan:
  proves receiver storage residence:
    direct_array / exact_native_struct / scalarized / public_arraybox_fallback

Backend:
  consumes the selected residence proof.
  It does not reinterpret public handles and does not infer Rust ArrayBox layout.

Public ArrayBox:
  remains the public facade and fallback materialization route.
```

## Accepted Proof Shape

A future inventory row may open implementation only when a surface row and
inventory prove:

```text
array_receiver_route_kind=array_slot_len
array_receiver_residence_owner=ObjectStoragePlan|ArrayRepr
array_receiver_residence=direct_array|exact_native_struct|scalarized
array_receiver_direct_facts_proven=1
array_receiver_materialization_route_known=1
array_receiver_public_handle_reinterpreted=0
array_receiver_backend_raw_layout_inference=0
array_receiver_host_handle_publication_before_read=0
array_receiver_fallback_public_arraybox=1
selected_storage_candidate_confidence=high
```

If the residence is `public_arraybox_fallback` or `none`, the hot route remains
on the generic HostHandle path.

## Rejected Designs

```text
reject: route proof as storage proof
  reason: array_slot_len says what operation is called, not where receiver
  storage lives

reject: public ArrayBox handle reinterpretation
  reason: ArrayBox remains the public facade and materialized view

reject: backend raw ArrayBox layout inference
  reason: backend cannot own Rust runtime ArrayBox layout truth

reject: nyash.array.birth_h behavior change
  reason: public birth remains ArrayBox; DirectArray / ArrayRepr residence must
  be an explicit representation plan

reject: MIRBuilder object management
  reason: MIRBuilder records source meaning; storage representation belongs to
  plans and exact-AOT backend consumers
```

## Stop Line

```text
do not implement backend direct handle bypass from this row
do not edit nyash_array_length_h from this row
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
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-SURFACE-001:
  define the concrete report fields for array_receiver_residence proof,
  including owner, residence, direct facts source, materialization route,
  fallback, and the open gate for a later high-confidence inventory row
```
