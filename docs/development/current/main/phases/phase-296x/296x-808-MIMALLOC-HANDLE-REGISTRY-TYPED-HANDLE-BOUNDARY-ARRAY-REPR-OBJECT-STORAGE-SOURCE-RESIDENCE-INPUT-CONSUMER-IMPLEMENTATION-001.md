---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-IMPLEMENTATION-001
Scope: Implement passive ArrayReceiverResidenceInput vocabulary and constructor
  from ArrayReceiverResidenceInputSource.
Related:
  - docs/development/current/main/phases/phase-296x/296x-807-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-SURFACE-001.md
  - docs/development/current/main/phases/phase-296x/array-receiver-residence-proof-chain.md
  - src/array_receiver_representation_source.rs
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-IMPLEMENTATION-001

## Purpose

Implement only the passive fallback `ArrayReceiverResidenceInput` vocabulary
opened by 296x-807.

This row does not create a backend-consumable residence fact. It records the
fallback/public ArrayBox materialization boundary so later rows can decide
whether a separate fact producer is useful.

## Implementation Report

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-residence-input-consumer-implementation-v0
source_evidence=296x-807,296x-806,296x-805,296x-804,array-repr-proof-chain-guide
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

implementation_module=src/array_receiver_representation_source.rs
residence_input_defined=1
residence_candidate_defined=1
consumer_input=ArrayReceiverResidenceInputSource
consumer_input_entry=ArrayReceiverResidenceProofChain
consumer_output=ArrayReceiverResidenceInput|none
consumer_constructor=ArrayReceiverResidenceInput::from_input_source
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

input_public_handle_reinterpretation=0
input_backend_raw_layout_inference=0
input_helper_name_inference=0
input_mirbuilder_owner=0
input_exported_to_mir_json=0
input_consumed_by_backend=0
input_report_fields_defined=1

backend_direct_handle_bypass_enabled=0
implementation_allowed=1
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
mirbuilder_object_management_enabled=0
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-CLOSEOUT-001
summary=ok
```

## Implemented Shape

```text
ArrayReceiverResidenceInputSource(PublicArrayBoxFallback)
  -> ArrayReceiverResidenceInput(public_arraybox_fallback)
```

The constructor rejects any direct-storage or backend-bypass authorization.

## Stop Line

```text
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
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-CLOSEOUT-001:
  close the input consumer stage and decide whether to proceed to a
  ResidenceFact producer inventory or stop for design consultation before
  backend consumption.
```
