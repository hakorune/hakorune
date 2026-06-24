---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-CONSUMER-IMPLEMENTATION-DESIGN-001
Scope: Design the passive fallback-only consumer implementation for
  ArrayReceiverConstructorHandoff.
Related:
  - docs/development/current/main/phases/phase-296x/296x-798-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-CONSUMER-INVENTORY-001.md
  - docs/development/current/main/phases/phase-296x/296x-797-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-IMPLEMENTATION-001.md
  - docs/development/current/main/phases/phase-296x/296x-790-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-CONSTRUCTION-SURFACE-001.md
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-CONSUMER-IMPLEMENTATION-DESIGN-001

## Purpose

296x-798 selected `ArrayReceiverResidenceSourceConstructor` as the only safe
consumer candidate for `ArrayReceiverConstructorHandoff`. This row designs the
implementation shape without connecting it yet.

The implementation must be fallback-only:

```text
ArrayReceiverConstructorHandoff(fallback_residence_candidate)
  -> ArrayReceiverResidenceSourceConstructor
  -> fallback input-source candidate
  -> no direct proof
  -> no backend bypass
```

## Decision

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-constructor-handoff-consumer-implementation-design-v0
source_evidence=296x-798,296x-797,296x-790,296x-789,array-repr-ssot,object-storage-plan-boundary-ssot
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

selected_design=array_receiver_residence_source_constructor_fallback_consumer
selected_design_confidence=medium
consumer_owner=RepresentationPlanner|ArrayReprSourcePlanner
consumer_input=ArrayReceiverConstructorHandoff
consumer_output=ArrayReceiverResidenceInputSource|none
consumer_scope=receiver_site_before_length_read
consumer_mode=fallback_only
consumer_accepts_fallback_residence_candidate=1
consumer_accepts_direct_residence_candidate=0
consumer_may_emit_fallback_input_source=1
consumer_may_emit_direct_input_source=0
consumer_preserves_public_arraybox_fallback=1
consumer_runtime_execution=0

required_input_handoff_kind=fallback_residence_candidate
required_input_materialization_route=public_arraybox_fallback
required_input_backend_bypass_authorized=0
required_input_direct_storage_proof=0

constructor_connection_allowed_next_row=1
source_exported_to_mir_json=0
source_consumed_by_backend=0
backend_direct_handle_bypass_enabled=0
implementation_allowed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
mirbuilder_object_management_enabled=0
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-CONSUMER-IMPLEMENTATION-001
summary=ok
```

## Implementation Shape

The next implementation row may add passive vocabulary such as:

```text
ArrayReceiverResidenceSourceConstructor
  input:
    ArrayReceiverConstructorHandoff

  output:
    Some(ArrayReceiverResidenceInputSource) only for fallback residence
    evidence that explicitly preserves public ArrayBox fallback semantics.

    None when the handoff claims direct storage, backend bypass, or an unknown
    materialization route that this row has not authorized.
```

This is still not backend lowering. It only names the residence source
candidate that later rows may feed into `ArrayReceiverResidenceInput`.

## Rejected Designs

```text
reject: consume handoff in backend
  reason: backend consumes proven facts and must not construct representation
  residence from fallback evidence.

reject: consume handoff in MIRBuilder
  reason: MIRBuilder records source meaning; it does not own object residence.

reject: treat fallback handoff as DirectArray proof
  reason: PublicArrayBoxFallback is facade/fallback evidence only.

reject: export handoff to MIR JSON
  reason: the in-process proof chain is not yet a MIR contract.

reject: enable direct handle bypass from fallback input
  reason: no direct storage proof exists.
```

## Stop Line

```text
do not implement the consumer from this row
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
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-CONSUMER-IMPLEMENTATION-001:
  implement the passive fallback-only consumer vocabulary. It may connect
  ArrayReceiverConstructorHandoff to an ArrayReceiverResidenceSourceConstructor
  data shape, but must keep direct proof, MIR JSON export, backend consumption,
  and direct handle bypass closed.
```
