---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-CLOSEOUT-001
Scope: Close the Array receiver residence proof-chain thinning detour and
  return the lane to the next `ArrayReceiverResidenceInput` consumer row.
Related:
  - docs/development/current/main/phases/phase-296x/296x-804-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-IMPLEMENTATION-001.md
  - docs/development/current/main/phases/phase-296x/array-receiver-residence-proof-chain.md
  - src/array_receiver_representation_source.rs
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-CLOSEOUT-001

## Purpose

Close the short BoxShape detour introduced after 296x-801. The proof chain was
not wrong, but it had too many adjacent nouns for future patches:

```text
RepresentationSource
  -> ConstructorHandoff
  -> ResidenceInputSource
  -> ResidenceInput
  -> ResidenceFact
```

The durable developer entry is now:

```text
ArrayReceiverResidenceProofChain
  .construct_input_source_from_representation_source(source)
```

`ArrayReceiverConstructorHandoff` stays as compatibility vocabulary for landed
296x-796..800 report gates. It is no longer the primary mental model.

## Closeout Report

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-residence-chain-thinning-closeout-v0
source_evidence=296x-801,296x-802,296x-803,296x-804,array-repr-proof-chain-guide
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

thinning_detour_closed=1
developer_facing_entry=ArrayReceiverResidenceProofChain
developer_guide=docs/development/current/main/phases/phase-296x/array-receiver-residence-proof-chain.md
primary_entry_method=construct_input_source_from_representation_source
primary_flow=ArrayReceiverRepresentationSource->ArrayReceiverResidenceProofChain->ArrayReceiverResidenceInputSource->ArrayReceiverResidenceInput

constructor_handoff_primary_mental_model=0
constructor_handoff_compat_kept=1
constructor_handoff_report_gates_preserved=1
stage_reports_preserved=1
proof_gates_collapsed=0
facade_adds_direct_proof_power=0

fallback_source_is_not_direct_proof=1
public_arraybox_fallback_acceptance=1
direct_storage_source_acceptance=0
backend_direct_handle_bypass_enabled=0
mir_json_export_enabled=0
backend_consumption_enabled=0
mirbuilder_object_management_enabled=0

next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-DESIGN-001
next_task_uses_entry=ArrayReceiverResidenceProofChain
next_task_target=ArrayReceiverResidenceInput
implementation_allowed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
summary=ok
```

## Readable Model

New work should read the chain as:

```text
representation source
  -> proof-chain facade
  -> fallback input-source
  -> residence input
```

Not as:

```text
representation source
  -> constructor handoff
  -> input source
```

The second form still exists in code only to keep the earlier report rows and
guards stable.

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
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-DESIGN-001:
  design the next fallback-only consumer step from
  ArrayReceiverResidenceInputSource to ArrayReceiverResidenceInput, using
  ArrayReceiverResidenceProofChain as the entry and keeping backend/direct
  routes closed.
```
