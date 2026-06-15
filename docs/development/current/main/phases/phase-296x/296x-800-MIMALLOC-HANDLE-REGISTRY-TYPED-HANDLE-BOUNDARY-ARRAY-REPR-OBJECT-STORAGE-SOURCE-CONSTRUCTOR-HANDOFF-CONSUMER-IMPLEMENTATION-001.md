---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-CONSUMER-IMPLEMENTATION-001
Scope: Implement passive fallback-only consumer vocabulary for
  ArrayReceiverConstructorHandoff.
Related:
  - docs/development/current/main/phases/phase-296x/296x-799-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-CONSUMER-IMPLEMENTATION-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/296x-798-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-CONSUMER-INVENTORY-001.md
  - src/array_receiver_representation_source.rs
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-CONSUMER-IMPLEMENTATION-001

## Purpose

296x-799 selected a fallback-only consumer implementation shape. This row
implements only passive vocabulary in `src/array_receiver_representation_source.rs`.

It connects source handoff to constructor vocabulary, but not to MIR JSON,
backend lowering, or direct handle bypass.

## Implementation Report

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-constructor-handoff-consumer-implementation-v0
source_evidence=296x-799,296x-798,296x-797,array-repr-ssot,object-storage-plan-boundary-ssot
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

implementation_module=src/array_receiver_representation_source.rs
residence_input_source_kind_defined=1
residence_input_source_struct_defined=1
residence_source_constructor_defined=1
constructor_input=ArrayReceiverConstructorHandoff
constructor_output=ArrayReceiverResidenceInputSource|none
constructor_mode=fallback_only
constructor_accepts_fallback_residence_candidate=1
constructor_accepts_direct_residence_candidate=0
constructor_preserves_public_arraybox_fallback=1
constructor_output_direct_storage_proof=0
constructor_output_backend_bypass_authorized=0
constructor_report_fields_defined=1

source_connected_to_constructor=1
source_exported_to_mir_json=0
source_consumed_by_backend=0
backend_direct_handle_bypass_enabled=0
implementation_allowed=1
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
mirbuilder_object_management_enabled=0
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-INPUT-SOURCE-CONSUMER-INVENTORY-001
summary=ok
```

## Code Contract

```text
ArrayReceiverResidenceSourceConstructor::construct(handoff):
  if handoff.kind == fallback_residence_candidate
  and materialization_route == public_arraybox_fallback
  and direct_storage_proof == false
  and backend_bypass_authorized == false:
    Some(ArrayReceiverResidenceInputSource(PublicArrayBoxFallback))

  otherwise:
    None
```

## Stop Line

```text
do not implement ArrayReceiverResidenceInput from this row
do not implement ArrayReceiverResidenceFact producer from this row
do not implement backend direct handle bypass from this row
do not export ArrayReceiverResidenceInputSource to MIR JSON from this row
do not consume ArrayReceiverResidenceInputSource in backend from this row
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
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-INPUT-SOURCE-CONSUMER-INVENTORY-001:
  inventory where ArrayReceiverResidenceInputSource can flow next. Keep MIR JSON,
  backend consumption, direct residence proof, and direct handle bypass closed
  unless a separate proof row opens them.
```
