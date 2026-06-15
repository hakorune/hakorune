---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-DESIGN-001
Scope: Reconnect the existing ArrayReceiverResidenceInput design to the new
  ArrayReceiverResidenceProofChain entry before implementation.
Related:
  - docs/development/current/main/phases/phase-296x/296x-805-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-CLOSEOUT-001.md
  - docs/development/current/main/phases/phase-296x/296x-784-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SURFACE-001.md
  - docs/development/current/main/phases/phase-296x/array-receiver-residence-proof-chain.md
  - src/array_receiver_representation_source.rs
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-DESIGN-001

## Purpose

296x-783 and 296x-784 defined `ArrayReceiverResidenceInput` before the later
`ArrayReceiverRepresentationSource` / proof-chain rows existed. 296x-805 closed
the thinning detour and made this the current entry:

```text
ArrayReceiverResidenceProofChain
  -> ArrayReceiverResidenceInputSource
```

This row reconnects the old `ArrayReceiverResidenceInput` design to the new
entry before implementation. It does not implement `ArrayReceiverResidenceInput`.

## Decision

`ArrayReceiverResidenceInput` is still the right next stage, but its immediate
input is now the proof-chain facade output:

```text
ArrayReceiverRepresentationSource
  -> ArrayReceiverResidenceProofChain
  -> ArrayReceiverResidenceInputSource
  -> ArrayReceiverResidenceInput
```

The first implementation remains fallback-only:

```text
PublicArrayBoxFallback input-source
  -> fallback residence input
```

This is not direct storage proof and does not authorize backend direct handle
bypass.

## Design Report

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-residence-input-consumer-design-v0
source_evidence=296x-805,296x-804,296x-801,296x-784,296x-783,array-repr-proof-chain-guide
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

selected_consumer=ArrayReceiverResidenceInput
consumer_owner=RepresentationPlanner|ObjectStoragePlan|ArrayRepr
consumer_input=ArrayReceiverResidenceInputSource
consumer_input_entry=ArrayReceiverResidenceProofChain
consumer_output=ArrayReceiverResidenceInput|none
consumer_mode=fallback_only
consumer_accepts_public_arraybox_fallback=1
consumer_accepts_direct_storage_source=0
consumer_preserves_materialization_route=public_arraybox_fallback

residence_input_surface_reused=1
residence_input_surface_source=296x-784
residence_input_direct_array_access_plan_available=0
residence_input_object_storage_plan_available=0
residence_input_array_repr_available=1
residence_input_escape_facts_available=0
residence_input_candidate=public_arraybox_fallback
residence_input_direct_storage_proof=0
residence_input_backend_bypass_authorized=0

fallback_source_is_not_direct_proof=1
proof_chain_entry_required=1
constructor_handoff_primary_mental_model=0
constructor_handoff_compat_kept=1
backend_direct_handle_bypass_enabled=0
mir_json_export_enabled=0
backend_consumption_enabled=0

implementation_allowed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
mirbuilder_object_management_enabled=0
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-SURFACE-001
summary=ok
```

## Field Mapping

The later surface / implementation row should map the fallback source into
`ArrayReceiverResidenceInput` like this:

```text
ArrayReceiverResidenceInput:
  receiver_site_id=none
  route_kind=array_slot_len
  receiver_box_name=ArrayBox
  direct_array_plan_available=0
  object_storage_plan_available=0
  array_repr_available=1
  residence_candidate=public_arraybox_fallback
  escape_facts_available=0
  host_handle_publication_before_read=1
  materialization_route_candidate=public_arraybox_fallback
  direct_storage_proof=0
  backend_bypass_authorized=0
```

`receiver_site_id=none` is intentional for this row. The current proof chain is
not yet bound to MIR JSON or backend callsite IDs.

## Rejected Designs

```text
reject: treat PublicArrayBoxFallback as direct storage proof
  reason: public ArrayBox is still the safe materialization/fallback boundary

reject: consume ArrayReceiverResidenceInputSource in backend
  reason: backend consumption needs a separate proof row

reject: infer Rust ArrayBox layout from the helper name
  reason: nyash_array_length_h is a symptom, not representation truth

reject: move the residence input owner into MIRBuilder
  reason: MIRBuilder records source meaning; representation residence is a
  planner/object-storage concern
```

## Stop Line

```text
do not implement ArrayReceiverResidenceInput from this row
do not implement ArrayReceiverResidenceFact producer from this row
do not implement backend direct handle bypass from this row
do not export ArrayReceiverResidenceInputSource to MIR JSON from this row
do not consume ArrayReceiverResidenceInputSource in backend from this row
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
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-SURFACE-001:
  define the concrete report fields and open gate for constructing
  ArrayReceiverResidenceInput from ArrayReceiverResidenceInputSource.
```
