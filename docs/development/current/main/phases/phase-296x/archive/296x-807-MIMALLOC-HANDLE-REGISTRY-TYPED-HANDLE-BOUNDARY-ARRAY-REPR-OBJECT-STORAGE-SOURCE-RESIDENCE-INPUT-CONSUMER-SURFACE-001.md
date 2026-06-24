---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-SURFACE-001
Scope: Define the report surface and open gate for constructing
  ArrayReceiverResidenceInput from ArrayReceiverResidenceInputSource.
Related:
  - docs/development/current/main/phases/phase-296x/296x-806-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/array-receiver-residence-proof-chain.md
  - src/array_receiver_representation_source.rs
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-SURFACE-001

## Purpose

296x-806 reconnected the existing `ArrayReceiverResidenceInput` design to the
current `ArrayReceiverResidenceProofChain` facade. This row defines the concrete
surface for a later implementation row.

This is still not backend proof. It only opens construction of a passive
fallback `ArrayReceiverResidenceInput`.

## Surface

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-residence-input-consumer-surface-v0
source_evidence=296x-806,296x-805,296x-804,296x-784,array-repr-proof-chain-guide
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

residence_input_consumer_surface_defined=1
consumer_input=ArrayReceiverResidenceInputSource
consumer_input_entry=ArrayReceiverResidenceProofChain
consumer_output=ArrayReceiverResidenceInput|none
consumer_owner=RepresentationPlanner|ObjectStoragePlan|ArrayRepr
consumer_mode=fallback_only
consumer_accepts_public_arraybox_fallback=1
consumer_accepts_direct_storage_source=0

input_field_receiver_site_id=none
input_field_route_kind=array_slot_len
input_field_receiver_box_name=ArrayBox
input_field_direct_array_plan_available=0
input_field_object_storage_plan_available=0
input_field_array_repr_available=1
input_field_residence_candidate=public_arraybox_fallback
input_field_escape_facts_available=0
input_field_host_handle_publication_before_read=1
input_field_materialization_route_candidate=public_arraybox_fallback
input_field_direct_storage_proof=0
input_field_backend_bypass_authorized=0

input_preserves_public_arraybox_fallback=1
input_public_handle_reinterpretation=0
input_backend_raw_layout_inference=0
input_helper_name_inference=0
input_mirbuilder_owner=0
input_exported_to_mir_json=0
input_consumed_by_backend=0

backend_direct_handle_bypass_enabled=0
implementation_allowed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
mirbuilder_object_management_enabled=0
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-IMPLEMENTATION-001
summary=ok
```

## Open Gate

A later implementation row may add only this passive type and constructor:

```rust
pub struct ArrayReceiverResidenceInput {
    pub receiver_site_id: Option<u32>,
    pub route_kind: &'static str,
    pub receiver_box_name: &'static str,
    pub direct_array_plan_available: bool,
    pub object_storage_plan_available: bool,
    pub array_repr_available: bool,
    pub residence_candidate: ArrayReceiverResidenceCandidate,
    pub escape_facts_available: bool,
    pub host_handle_publication_before_read: bool,
    pub materialization_route_candidate: ArrayReceiverMaterializationRoute,
    pub direct_storage_proof: bool,
    pub backend_bypass_authorized: bool,
}

pub enum ArrayReceiverResidenceCandidate {
    PublicArrayBoxFallback,
}

impl ArrayReceiverResidenceInput {
    pub fn from_input_source(
        source: &ArrayReceiverResidenceInputSource,
    ) -> Option<Self>;
}
```

The implementation must preserve:

```text
input_field_direct_storage_proof=0
input_field_backend_bypass_authorized=0
input_public_handle_reinterpretation=0
input_backend_raw_layout_inference=0
input_helper_name_inference=0
input_mirbuilder_owner=0
input_exported_to_mir_json=0
input_consumed_by_backend=0
backend_direct_handle_bypass_enabled=0
```

## Rejected Designs

```text
reject: use receiver_site_id as a required field
  reason: the current proof chain is not bound to MIR JSON or backend callsite
  IDs; forcing site identity now would add fake precision

reject: add DirectI64 residence candidate in this row
  reason: direct storage proof is still absent

reject: make ArrayReceiverResidenceInput a backend input
  reason: backend consumption needs a separate proof row

reject: infer ArrayBox layout from route_kind or helper symbol
  reason: route_kind=array_slot_len is route evidence, not storage proof
```

## Stop Line

```text
do not implement ArrayReceiverResidenceInput from this row
do not implement ArrayReceiverResidenceFact producer from this row
do not implement backend direct handle bypass from this row
do not export ArrayReceiverResidenceInput to MIR JSON from this row
do not consume ArrayReceiverResidenceInput in backend from this row
do not collapse proof gates without preserving report fields
do not treat PublicArrayBoxFallback as direct storage proof
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
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-IMPLEMENTATION-001:
  add the passive ArrayReceiverResidenceInput vocabulary and
  from_input_source constructor only. Keep backend/direct routes closed.
```
