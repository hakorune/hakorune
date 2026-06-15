---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-INPUT-SOURCE-CONSUMER-INVENTORY-001
Scope: Inventory the consumer for ArrayReceiverResidenceInputSource and check
  whether the residence proof chain needs a thinning row before more
  implementation.
Related:
  - docs/development/current/main/phases/phase-296x/296x-800-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-CONSUMER-IMPLEMENTATION-001.md
  - docs/development/current/main/phases/phase-296x/296x-786-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/296x-783-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-DESIGN-001.md
  - src/array_receiver_representation_source.rs
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-INPUT-SOURCE-CONSUMER-INVENTORY-001

## Purpose

296x-800 implemented passive fallback-only construction of
`ArrayReceiverResidenceInputSource`. This row inventories where that input
source can flow next and checks whether the current chain is still thin enough
to keep extending directly.

## Inventory

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-constructor-handoff-input-source-consumer-inventory-v0
source_evidence=296x-800,296x-786,296x-783,296x-780,array-repr-ssot,object-storage-plan-boundary-ssot
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

input_source_input=ArrayReceiverResidenceInputSource
input_source_kind=public_arraybox_fallback
input_source_direct_storage_proof=0
input_source_backend_bypass_authorized=0

input_source_consumer_candidate_count=1
input_source_consumer_candidate_0=ArrayReceiverResidenceInput
input_source_consumer_candidate_0_owner=RepresentationPlanner|ObjectStoragePlan|ArrayRepr
input_source_consumer_candidate_0_status=design_only
input_source_consumer_candidate_0_code_exists=0
input_source_consumer_candidate_0_accepts_fallback_source=1
input_source_consumer_candidate_0_accepts_direct_source=not_yet
input_source_consumer_candidate_0_may_emit=ArrayReceiverResidenceInput
input_source_consumer_candidate_0_must_preserve_public_arraybox_fallback=1

input_source_safe_to_connect_as_fallback_source=1
input_source_safe_to_connect_as_direct_residence_proof=0
input_source_safe_to_connect_to_backend=0
input_source_safe_to_export_to_mir_json=0
input_source_safe_to_enable_direct_handle_bypass=0

residence_chain_current_shape=RepresentationSource->ConstructorHandoff->InputSource->Input->Fact
residence_chain_stage_count=5
residence_chain_status=controlled_but_long
residence_chain_thinning_needed_before_next_implementation=1
residence_chain_thinning_goal=collapse_naming_and_owner_facade_without_collapsing_proof_gates

selected_consumer=ArrayReceiverResidenceInput
selected_consumer_confidence=medium
selected_consumer_mode=fallback_only
selected_next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-DESIGN-001

input_connection_allowed=0
producer_fact_connection_allowed=0
backend_direct_handle_bypass_enabled=0
implementation_allowed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
mirbuilder_object_management_enabled=0
summary=ok
```

## Reading

The consumer path is clear:

```text
ArrayReceiverResidenceInputSource
  -> ArrayReceiverResidenceInput
  -> ArrayReceiverResidenceFact
```

But the current proof chain has grown long enough that another implementation
row would add more vocabulary without first naming the facade. The chain is not
wrong: each stage prevents fallback evidence from being silently promoted into
direct storage proof. The readability risk is that future agents may see five
names and patch the nearest one.

Before implementing `ArrayReceiverResidenceInput`, add a thinness design row
that fixes one owner facade and naming map.

## Stop Line

```text
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
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-DESIGN-001:
  define the thin facade / naming map for the Array receiver residence proof
  chain before adding more implementation vocabulary. Preserve Source,
  InputSource, Input, and Fact report gates, but make one owner facade the
  developer-facing entry.
```
