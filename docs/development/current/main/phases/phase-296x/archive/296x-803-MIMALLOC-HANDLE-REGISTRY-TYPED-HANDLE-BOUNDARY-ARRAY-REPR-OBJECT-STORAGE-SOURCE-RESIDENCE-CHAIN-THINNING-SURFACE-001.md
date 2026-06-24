---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-SURFACE-001
Scope: Define the report surface and open gate for the
  ArrayReceiverResidenceProofChain facade.
Related:
  - docs/development/current/main/phases/phase-296x/296x-802-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/296x-801-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-INPUT-SOURCE-CONSUMER-INVENTORY-001.md
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-SURFACE-001

## Purpose

296x-802 selected `ArrayReceiverResidenceProofChain` as the developer-facing
facade for the staged Array receiver residence proof chain.

This row defines the concrete report surface and open gate for a later
implementation row. It does not implement the facade.

## Surface

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-residence-chain-thinning-surface-v0
source_evidence=296x-802,296x-801,296x-800,array-repr-ssot,object-storage-plan-boundary-ssot
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

residence_proof_chain_surface_defined=1
developer_facing_entry=ArrayReceiverResidenceProofChain
facade_owner=RepresentationPlanner|ArrayReprSourcePlanner
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
facade_materialization_route=public_arraybox_fallback

backend_direct_handle_bypass_enabled=0
implementation_allowed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
mirbuilder_object_management_enabled=0
summary=ok
```

## Open Gate

A later implementation row may add only this facade entry:

```text
ArrayReceiverResidenceProofChain::construct_input_source_from_representation_source(
  source: &ArrayReceiverRepresentationSource
) -> Option<ArrayReceiverResidenceInputSource>
```

The implementation may internally call the existing compatibility handoff path.
It must preserve all of:

```text
facade_keeps_constructor_handoff_compat=1
facade_preserves_stage_reports=1
facade_preserves_stop_lines=1
facade_adds_direct_proof_power=0
facade_exports_to_mir_json=0
facade_consumed_by_backend=0
facade_accepts_public_arraybox_fallback=1
facade_accepts_direct_storage_source=0
facade_output_direct_storage_proof=0
facade_output_backend_bypass_authorized=0
backend_direct_handle_bypass_enabled=0
```

## Stop Line

```text
do not implement the facade from this row
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
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-IMPLEMENTATION-001:
  implement the facade entry only. It should call the existing fallback-only
  path and keep all direct proof / backend / MIR JSON gates closed.
```
