---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-IMPLEMENTATION-001
Scope: Implement the passive fallback-only ArrayReceiverRepresentationSource
  vocabulary without enabling backend direct handle bypass.
Related:
  - docs/development/current/main/phases/phase-296x/296x-794-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-INVENTORY-001.md
  - docs/development/current/main/phases/phase-296x/296x-793-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-SURFACE-001.md
  - src/array_receiver_representation_source.rs
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-IMPLEMENTATION-001

## Purpose

296x-794 allowed only a fallback representation source implementation:
`ArrayRepr::PublicArrayBoxFallback`. This row implements that passive
vocabulary and report surface in Rust code.

It does not connect the source to MIR metadata, the residence constructor, or
backend lowering.

## Implementation Report

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-implementation-v0
source_evidence=296x-794,296x-793,296x-792,array-repr-ssot,object-storage-plan-boundary-ssot
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

array_receiver_representation_source_module=src/array_receiver_representation_source.rs
array_receiver_representation_source_defined=1
array_receiver_representation_source_exported=1
representation_source_owner=ArrayRepr
representation_source_output=ArrayReceiverRepresentationSource
representation_source_consumed_by=ArrayReceiverResidenceSourceConstructor
representation_source_route_kind=array_slot_len
representation_source_receiver_box_name=ArrayBox
representation_source_array_repr=PublicArrayBoxFallback
representation_source_object_storage_plan_ref=none
representation_source_direct_array_access_plan_ref=none
representation_source_materialization_route=public_arraybox_fallback
representation_source_confidence=high
representation_source_is_fallback_only=1
representation_source_proves_direct_storage=0
representation_source_authorizes_backend_bypass=0
representation_source_report_fields_defined=1

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
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-DESIGN-001
summary=ok
```

## Code Contract

```text
ArrayReceiverRepresentationSource::public_arraybox_fallback():
  owner=ArrayRepr
  route_kind=array_slot_len
  receiver_box_name=ArrayBox
  array_repr=PublicArrayBoxFallback
  object_storage_plan_ref=None
  direct_array_access_plan_ref=None
  materialization_route=public_arraybox_fallback
  confidence=high

is_fallback_only()=true
proves_direct_storage()=false
authorizes_backend_direct_handle_bypass()=false
```

## Reading

This is a proof-chain cleanup, not a speed row.

```text
now explicit:
  fallback ArrayBox/materialized residence proof

still missing:
  DirectI64 or ObjectStoragePlan direct residence proof

still closed:
  backend direct handle bypass
```

The next row should decide how
`ArrayReceiverResidenceSourceConstructor` consumes this fallback source without
claiming a direct storage proof.

## Stop Line

```text
do not connect ArrayReceiverRepresentationSource to backend lowering from this row
do not export ArrayReceiverRepresentationSource to MIR JSON from this row
do not implement ArrayReceiverResidenceSourceConstructor from this row
do not implement ArrayReceiverResidenceInputSource from this row
do not implement ArrayReceiverResidenceInput from this row
do not implement ArrayReceiverResidenceFact producer from this row
do not implement backend direct handle bypass from this row
do not reinterpret public ArrayBox HostHandle as direct storage
do not infer Rust ArrayBox layout in backend
do not infer representation from helper name
do not treat PublicArrayBoxFallback as direct storage proof
do not treat DirectArrayAccessPlan alone as direct storage proof
do not move Box/Object management into MIRBuilder
do not change nyash.array.birth_h public semantics
do not retire HostHandle globally
do not retire Arc globally
```

## Next

```text
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-DESIGN-001:
  design how ArrayReceiverResidenceSourceConstructor consumes the passive
  fallback source while preserving the direct-bypass gate.
```
