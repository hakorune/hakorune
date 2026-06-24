---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-IMPLEMENTATION-001
Scope: Implement the ArrayReceiverResidenceProofChain facade entry while
  preserving existing proof gates and keeping backend/direct routes closed.
Related:
  - docs/development/current/main/phases/phase-296x/296x-803-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-SURFACE-001.md
  - docs/development/current/main/phases/phase-296x/296x-802-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-DESIGN-001.md
  - src/array_receiver_representation_source.rs
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-IMPLEMENTATION-001

## Purpose

296x-803 fixed the facade open gate. This row implements only the
developer-facing entry:

```text
ArrayReceiverResidenceProofChain::construct_input_source_from_representation_source(...)
```

The facade calls the existing fallback-only compatibility path internally. It
does not add proof power.

## Implementation Report

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-residence-chain-thinning-implementation-v0
source_evidence=296x-803,296x-802,296x-800,array-repr-ssot,object-storage-plan-boundary-ssot
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

implementation_module=src/array_receiver_representation_source.rs
developer_facing_entry=ArrayReceiverResidenceProofChain
facade_input=ArrayReceiverRepresentationSource
facade_output=ArrayReceiverResidenceInputSource|none
facade_first_method=construct_input_source_from_representation_source
facade_keeps_constructor_handoff_compat=1
facade_hides_constructor_handoff_from_primary_docs=1
facade_preserves_stage_reports=1
facade_preserves_stop_lines=1
facade_adds_direct_proof_power=0
facade_exports_to_mir_json=0
facade_consumed_by_backend=0

facade_accepts_public_arraybox_fallback=1
facade_accepts_direct_storage_source=0
facade_output_direct_storage_proof=0
facade_output_backend_bypass_authorized=0
facade_report_fields_defined=1

backend_direct_handle_bypass_enabled=0
implementation_allowed=1
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
mirbuilder_object_management_enabled=0
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-CLOSEOUT-001
summary=ok
```

## Developer Guide

Use this entry when reading or extending the chain:

```text
ArrayReceiverResidenceProofChain
  .construct_input_source_from_representation_source(source)
```

Avoid starting from `ArrayReceiverConstructorHandoff` in new work. It remains
only as internal compatibility vocabulary for old report gates.

## Stop Line

```text
do not delete ArrayReceiverConstructorHandoff from this row
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
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-CLOSEOUT-001:
  close the thinning detour, record that the facade is the developer-facing
  entry, then return to the ArrayReceiverResidenceInput consumer row.
```
