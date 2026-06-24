---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-IMPLEMENTATION-001
Scope: Implement passive fallback-source constructor handoff vocabulary without
  connecting it to constructor execution, MIR JSON, or backend direct handle
  bypass.
Related:
  - docs/development/current/main/phases/phase-296x/296x-796-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/296x-795-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-IMPLEMENTATION-001.md
  - src/array_receiver_representation_source.rs
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-IMPLEMENTATION-001

## Purpose

296x-796 fixed the design: a `PublicArrayBoxFallback` representation source may
flow to the constructor as fallback residence evidence, but it must not become
direct storage proof or authorize backend direct handle bypass.

This row implements only that passive vocabulary.

## Implementation Report

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-constructor-handoff-implementation-v0
source_evidence=296x-796,296x-795,296x-794,array-repr-ssot,object-storage-plan-boundary-ssot
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

handoff_module=src/array_receiver_representation_source.rs
handoff_kind_defined=1
handoff_struct_defined=1
handoff_input=ArrayReceiverRepresentationSource
handoff_consumer=ArrayReceiverResidenceSourceConstructor
handoff_output_kind=fallback_residence_candidate
handoff_input_array_repr=PublicArrayBoxFallback
handoff_input_is_fallback_only=1
handoff_output_direct_storage_proof=0
handoff_output_backend_bypass_authorized=0
handoff_materialization_route=public_arraybox_fallback
handoff_preserves_public_arraybox_fallback=1
handoff_report_fields_defined=1

source_connected_to_constructor=0
source_exported_to_mir_json=0
source_consumed_by_backend=0
backend_direct_handle_bypass_enabled=0
implementation_allowed=1
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
mirbuilder_object_management_enabled=0
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-CONSUMER-INVENTORY-001
summary=ok
```

## Code Contract

```text
ArrayReceiverRepresentationSource::public_arraybox_fallback()
  .constructor_handoff():
    kind=fallback_residence_candidate
    materialization_route=public_arraybox_fallback
    direct_storage_proof=false
    backend_bypass_authorized=false
```

The implementation is deliberately data-only. It does not inspect MIR and does
not perform residence construction.

## Stop Line

```text
do not connect constructor handoff to ArrayReceiverResidenceSourceConstructor from this row
do not implement ArrayReceiverResidenceInputSource from this row
do not implement ArrayReceiverResidenceInput from this row
do not implement ArrayReceiverResidenceFact producer from this row
do not implement backend direct handle bypass from this row
do not export ArrayReceiverRepresentationSource to MIR JSON from this row
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
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-CONSUMER-INVENTORY-001:
  inventory where the passive handoff can be consumed next. Keep backend direct
  handle bypass closed unless a direct residence proof appears.
```
