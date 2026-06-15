---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-FACT-BOUNDARY-DESIGN-001
Scope: Decide whether fallback-only ArrayReceiverResidenceInput may produce
  ResidenceFact.
Related:
  - docs/development/current/main/phases/phase-296x/296x-809-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-CLOSEOUT-001.md
  - docs/development/current/main/phases/phase-296x/array-receiver-residence-proof-chain.md
  - src/array_receiver_representation_source.rs
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-FACT-BOUNDARY-DESIGN-001

## Purpose

Close the design-consultation point from 296x-809 without adding another
report-only layer.

The decision is:

```text
Fallback-only public ArrayBox evidence stops at ArrayReceiverResidenceInput.
ResidenceFact is reserved for direct, backend-consumable proof.
```

This keeps the chain thin:

```text
fallback:
  ArrayReceiverResidenceInput
  -> stop

direct:
  ArrayReceiverResidenceInput
  -> DirectResidenceFact
  -> backend-consumable consumer
```

## Decision Report

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-residence-fact-boundary-design-v0
decision=B_plus_C_lite
source_evidence=296x-809,296x-808,array-repr-proof-chain-guide
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

closed_stage=ArrayReceiverResidenceInput
closed_stage_mode=fallback_only
closed_stage_backend_consumable=0
array_receiver_residence_input_backend_consumable=0

fallback_fact_producer_enabled=0
fallback_residence_fact_enabled=0
fallback_residence_fact_reserved=0
public_arraybox_fallback_fact_produced=0
public_arraybox_fallback_is_negative_evidence=1

direct_residence_fact_reserved=1
residence_fact_requires_direct_storage_proof=1
residence_fact_requires_backend_bypass_authorization=1
residence_fact_backend_consumable=1

backend_reads_input=0
backend_reads_fact=1
backend_direct_handle_bypass_enabled=0
mir_json_export_enabled=0
backend_consumption_enabled=0
mirbuilder_object_management_enabled=0

implementation_allowed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0

next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-DIRECT-RESIDENCE-PROOF-OWNER-SELECTION-001
summary=ok
```

## Boundary

`Input` is a downstream-safe evidence carrier:

```text
Input:
  may carry fallback evidence
  is not backend-consumable
  is not exported to MIR JSON
  is not read by backend lowering
```

`Fact` is a direct proof carrier:

```text
Fact:
  requires direct storage proof
  requires backend bypass authorization
  may be consumed by verifier / planner / backend
```

Therefore:

```text
PublicArrayBoxFallback:
  Input only

DirectI64 / exact ObjectStoragePlan:
  may later produce DirectResidenceFact
```

## Stop Line

```text
do not implement ArrayReceiverResidenceFact from fallback-only evidence
do not create report-only Fact with backend-like name
do not create FallbackResidenceFact
do not let backend read ArrayReceiverResidenceInput
do not export fallback residence input to MIR JSON
do not treat PublicArrayBoxFallback as direct storage proof
do not reinterpret public ArrayBox HostHandle as raw ArrayBox layout
do not infer representation from helper name nyash_array_length_h
do not move Box/Object management into MIRBuilder
do not implement backend direct handle bypass from this row
do not retire HostHandle globally
do not retire Arc globally
```

## Next Row

The next row should not implement a fact producer. It should select the owner
that can produce direct proof later:

```text
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-DIRECT-RESIDENCE-PROOF-OWNER-SELECTION-001
```

Candidate owner families:

```text
ObjectStoragePlan direct residence
ArrayRepr::DirectI64 source
DirectArray birth/source tracking
escape/publication proof
backend consumer metadata
```

`DirectArrayAccessPlan` alone is not enough. It is access-site evidence, not
receiver storage proof.
