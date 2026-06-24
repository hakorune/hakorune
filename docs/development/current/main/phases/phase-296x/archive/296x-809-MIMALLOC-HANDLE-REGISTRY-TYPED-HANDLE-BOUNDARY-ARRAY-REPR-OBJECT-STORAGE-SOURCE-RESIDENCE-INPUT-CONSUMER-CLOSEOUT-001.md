---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-CLOSEOUT-001
Scope: Close the passive ArrayReceiverResidenceInput consumer stage and stop
  before ResidenceFact / backend-consumption design.
Related:
  - docs/development/current/main/phases/phase-296x/296x-808-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-IMPLEMENTATION-001.md
  - docs/development/current/main/phases/phase-296x/array-receiver-residence-proof-chain.md
  - src/array_receiver_representation_source.rs
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-CLOSEOUT-001

## Purpose

Close the passive residence input stage:

```text
ArrayReceiverRepresentationSource
  -> ArrayReceiverResidenceProofChain
  -> ArrayReceiverResidenceInputSource
  -> ArrayReceiverResidenceInput
```

The next possible stage is `ArrayReceiverResidenceFact`, but that is where the
lane approaches backend-consumable proof. That is a design-consultation point,
not an automatic implementation continuation.

## Closeout Report

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-residence-input-consumer-closeout-v0
source_evidence=296x-808,296x-807,296x-806,296x-805,array-repr-proof-chain-guide
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

residence_input_consumer_closed=1
developer_facing_entry=ArrayReceiverResidenceProofChain
closed_flow=ArrayReceiverRepresentationSource->ArrayReceiverResidenceProofChain->ArrayReceiverResidenceInputSource->ArrayReceiverResidenceInput
closed_stage=ArrayReceiverResidenceInput
closed_stage_mode=fallback_only
closed_stage_backend_consumable=0
closed_stage_direct_storage_proof=0
closed_stage_backend_bypass_authorized=0

residence_fact_producer_implemented=0
residence_fact_backend_consumable=0
backend_direct_handle_bypass_enabled=0
mir_json_export_enabled=0
backend_consumption_enabled=0
mirbuilder_object_management_enabled=0

next_stage=ArrayReceiverResidenceFact
next_stage_requires_design_consultation=1
next_stage_reason=backend_consumable_proof_boundary
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-FACT-PRODUCER-DESIGN-CONSULTATION-001

implementation_allowed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
summary=ok
```

## Current Boundary

The current implementation can say:

```text
This Array receiver is represented as the public ArrayBox fallback route.
This is safe fallback residence input.
This is not direct storage proof.
This is not backend direct-handle bypass authorization.
```

It cannot say:

```text
The backend may bypass HostHandle lookup.
The backend knows raw ArrayBox layout.
The proof is direct residence.
The proof is MIR JSON / backend consumable.
```

## Stop Line

```text
do not implement ArrayReceiverResidenceFact producer from this row
do not implement backend direct handle bypass from this row
do not export ArrayReceiverResidenceInput to MIR JSON from this row
do not consume ArrayReceiverResidenceInput in backend from this row
do not treat PublicArrayBoxFallback as direct storage proof
do not reinterpret public ArrayBox HostHandle as direct storage
do not infer Rust ArrayBox layout in backend
do not infer representation from helper name
do not move Box/Object management into MIRBuilder
do not change nyash.array.birth_h public semantics
do not retire HostHandle globally
do not retire Arc globally
```

## Design Consultation Prompt

Ask before opening the next stage:

```text
Should ArrayReceiverResidenceFact exist for fallback-only public ArrayBox
evidence, or should the lane stop here until direct ObjectStoragePlan /
ArrayRepr evidence exists?

If it should exist, is it:
  A. report-only fact, never backend-consumable
  B. backend-consumable only when direct storage evidence appears later
  C. split into FallbackResidenceFact and DirectResidenceFact
```

Do not continue into backend consumption without this decision.
