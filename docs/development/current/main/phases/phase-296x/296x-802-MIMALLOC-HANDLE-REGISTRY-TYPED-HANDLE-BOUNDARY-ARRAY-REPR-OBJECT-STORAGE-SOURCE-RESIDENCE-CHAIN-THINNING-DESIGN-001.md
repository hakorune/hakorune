---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-DESIGN-001
Scope: Define a thin developer-facing facade / naming map for the Array
  receiver residence proof chain before adding more implementation vocabulary.
Related:
  - docs/development/current/main/phases/phase-296x/296x-801-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-INPUT-SOURCE-CONSUMER-INVENTORY-001.md
  - docs/development/current/main/phases/phase-296x/296x-800-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-CONSUMER-IMPLEMENTATION-001.md
  - docs/development/current/main/phases/phase-296x/296x-786-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-DESIGN-001.md
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-DESIGN-001

## Purpose

296x-801 found the current proof chain is controlled but long:

```text
RepresentationSource -> ConstructorHandoff -> InputSource -> Input -> Fact
```

The stages are useful proof gates, but the public mental model should not keep
adding more nouns. This row keeps the gates and report fields, while selecting
a thin developer-facing facade.

## Decision

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-residence-chain-thinning-design-v0
source_evidence=296x-801,296x-800,296x-786,296x-783,296x-780,array-repr-ssot,object-storage-plan-boundary-ssot
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

selected_design=array_receiver_residence_proof_chain_facade
selected_design_confidence=medium
developer_facing_entry=ArrayReceiverResidenceProofChain
implementation_facade_owner=RepresentationPlanner|ArrayReprSourcePlanner
facade_first_method=construct_input_source_from_representation_source
facade_preserves_existing_stage_reports=1
facade_collapses_public_mental_model=1
facade_removes_proof_gates=0
facade_turns_fallback_into_direct_proof=0

stage_0=ArrayReceiverRepresentationSource
stage_0_role=upstream_representation_evidence
stage_1=ArrayReceiverConstructorHandoff
stage_1_role=compat_internal_handoff_not_primary_mental_model
stage_2=ArrayReceiverResidenceInputSource
stage_2_role=input_source_candidate
stage_3=ArrayReceiverResidenceInput
stage_3_role=producer_input
stage_4=ArrayReceiverResidenceFact
stage_4_role=backend_consumable_fact_later

constructor_handoff_public_primary_entry=0
constructor_handoff_keep_compat_vocabulary=1
constructor_handoff_can_be_private_later=1
construct_from_source_entry_allowed_next_row=1
direct_proof_path_allowed_next_row=0
backend_direct_handle_bypass_enabled=0
implementation_allowed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
mirbuilder_object_management_enabled=0
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-SURFACE-001
summary=ok
```

## Naming Map

Use this map in future cards and code reviews:

```text
Developer-facing:
  ArrayReceiverResidenceProofChain

Internal staged gates:
  RepresentationSource:
    what representation evidence exists?

  ConstructorHandoff:
    compatibility/internal handoff from older rows.
    Not the primary public mental model unless it starts carrying independent
    evidence.

  ResidenceInputSource:
    candidate source that can feed producer input.

  ResidenceInput:
    normalized producer input.

  ResidenceFact:
    later backend-consumable fact.
```

## Rejected Designs

```text
reject: delete ConstructorHandoff immediately
  reason: it is already guarded and tested; deletion should be a later
  compatibility cleanup after facade entry exists.

reject: collapse InputSource and Input into one type now
  reason: the older report gates distinguish source construction from producer
  input. Removing that distinction now would lose diagnostic boundary.

reject: continue adding public nouns without a facade
  reason: the patch surface is already long enough to invite local fixes in the
  nearest type.

reject: open backend direct handle bypass as part of thinning
  reason: thinning is BoxShape only. It must not add proof power.
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
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-SURFACE-001:
  define concrete facade report fields and open gate. The next implementation
  may add construct_input_source_from_representation_source, but it must keep
  the existing stage reports and direct/backend gates closed.
```
