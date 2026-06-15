---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-CONSUMER-INVENTORY-001
Scope: Inventory the current consumer surface for the passive
  ArrayReceiverConstructorHandoff before connecting it to residence
  construction.
Related:
  - docs/development/current/main/phases/phase-296x/296x-797-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-IMPLEMENTATION-001.md
  - docs/development/current/main/phases/phase-296x/296x-796-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/296x-791-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-CONSTRUCTION-INVENTORY-001.md
  - src/array_receiver_representation_source.rs
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-CONSUMER-INVENTORY-001

## Purpose

296x-797 added passive `ArrayReceiverConstructorHandoff` vocabulary. This row
checks where that handoff can be consumed next.

The current handoff is safe only as a fallback residence candidate:

```text
PublicArrayBoxFallback source
  -> constructor handoff
  -> fallback_residence_candidate
  -> no direct storage proof
  -> no backend direct handle bypass
```

## Inventory

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-constructor-handoff-consumer-inventory-v0
source_evidence=296x-797,296x-796,296x-791,296x-789,array-repr-ssot,object-storage-plan-boundary-ssot
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

handoff_input=ArrayReceiverConstructorHandoff
handoff_input_kind=fallback_residence_candidate
handoff_input_array_repr=PublicArrayBoxFallback
handoff_input_direct_storage_proof=0
handoff_input_backend_bypass_authorized=0

handoff_consumer_candidate_count=1
handoff_consumer_candidate_0=ArrayReceiverResidenceSourceConstructor
handoff_consumer_candidate_0_owner=RepresentationPlanner|ArrayReprSourcePlanner
handoff_consumer_candidate_0_status=design_only
handoff_consumer_candidate_0_code_exists=0
handoff_consumer_candidate_0_accepts_fallback_candidate=1
handoff_consumer_candidate_0_accepts_direct_candidate=not_yet
handoff_consumer_candidate_0_may_emit=ArrayReceiverResidenceInputSource
handoff_consumer_candidate_0_must_preserve_public_arraybox_fallback=1

handoff_safe_to_connect_as_fallback_candidate=1
handoff_safe_to_connect_as_direct_residence_proof=0
handoff_safe_to_connect_to_backend=0
handoff_safe_to_export_to_mir_json=0
handoff_safe_to_enable_direct_handle_bypass=0

selected_consumer=ArrayReceiverResidenceSourceConstructor
selected_consumer_confidence=medium
selected_consumer_mode=fallback_only
selected_next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-CONSUMER-IMPLEMENTATION-DESIGN-001

constructor_connection_allowed=0
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

There is exactly one intended consumer: `ArrayReceiverResidenceSourceConstructor`.
That consumer is still a design-only seam. Connecting the handoff directly in
this row would skip the constructor contract and risk turning fallback
evidence into execution proof.

The next row may design a passive consumer implementation, but it must remain
fallback-only until a separate direct representation proof exists.

## Stop Line

```text
do not connect constructor handoff to ArrayReceiverResidenceSourceConstructor from this row
do not implement ArrayReceiverResidenceInputSource from this row
do not implement ArrayReceiverResidenceInput from this row
do not implement ArrayReceiverResidenceFact producer from this row
do not implement backend direct handle bypass from this row
do not export ArrayReceiverConstructorHandoff to MIR JSON from this row
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
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-CONSUMER-IMPLEMENTATION-DESIGN-001:
  design the passive fallback-only consumer implementation. It may connect
  ArrayReceiverConstructorHandoff to ArrayReceiverResidenceSourceConstructor
  only as fallback residence evidence, with direct proof and backend bypass
  still closed.
```
