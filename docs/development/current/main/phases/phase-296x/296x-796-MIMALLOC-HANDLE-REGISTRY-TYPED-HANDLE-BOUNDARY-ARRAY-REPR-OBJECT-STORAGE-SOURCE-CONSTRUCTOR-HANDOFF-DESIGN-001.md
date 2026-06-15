---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-DESIGN-001
Scope: Design how ArrayReceiverResidenceSourceConstructor consumes the passive
  fallback-only ArrayReceiverRepresentationSource without opening backend direct
  handle bypass.
Related:
  - docs/development/current/main/phases/phase-296x/296x-795-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-IMPLEMENTATION-001.md
  - docs/development/current/main/phases/phase-296x/296x-794-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-INVENTORY-001.md
  - src/array_receiver_representation_source.rs
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-DESIGN-001

## Purpose

296x-795 implemented a passive `ArrayReceiverRepresentationSource` for
`PublicArrayBoxFallback`. The source is high-confidence evidence that the hot
receiver is the public/materialized ArrayBox fallback, but it explicitly does
not prove direct storage.

This row designs the next handoff:
`ArrayReceiverResidenceSourceConstructor` may consume this fallback-only source
as a fallback residence input candidate, while preserving the direct bypass
gate.

## Decision

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-constructor-handoff-design-v0
source_evidence=296x-795,296x-794,296x-793,array-repr-ssot,object-storage-plan-boundary-ssot
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

selected_design=fallback_source_constructor_handoff
selected_design_confidence=high
handoff_input=ArrayReceiverRepresentationSource
handoff_input_array_repr=PublicArrayBoxFallback
handoff_input_is_fallback_only=1
handoff_consumer=ArrayReceiverResidenceSourceConstructor
handoff_output_kind=fallback_residence_candidate
handoff_output_direct_storage_proof=0
handoff_output_backend_bypass_authorized=0
handoff_materialization_route=public_arraybox_fallback
handoff_preserves_public_arraybox_fallback=1
handoff_requires_direct_source_for_bypass=1
handoff_accepts_fallback_source=1
handoff_rejects_fallback_as_direct_source=1

source_connected_to_constructor=0
source_exported_to_mir_json=0
source_consumed_by_backend=0
backend_direct_handle_bypass_enabled=0
implementation_allowed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
mirbuilder_object_management_enabled=0
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-SURFACE-001
summary=ok
```

## Handoff Contract

The constructor receives representation evidence in two different categories.

```text
fallback residence candidate:
  PublicArrayBoxFallback
  materialization_route=public_arraybox_fallback
  direct_storage_proof=0
  backend_bypass_authorized=0

direct residence candidate:
  DirectI64 or ObjectStoragePlan direct/native/scalar evidence
  direct_storage_proof=1
  backend_bypass_authorized may be considered by later rows
```

`PublicArrayBoxFallback` is useful because it stops downstream code from
guessing. It tells the constructor that the only proven residence is the public
ArrayBox fallback path.

## Layer Contract

```text
ArrayReceiverRepresentationSource:
  names representation evidence
  can represent fallback-only materialization
  cannot authorize backend direct handle bypass

ArrayReceiverResidenceSourceConstructor:
  may consume fallback-only representation source
  may emit fallback residence candidate later
  must keep direct residence proof false for fallback-only input

Backend:
  still consumes later proven facts only
  does not infer layout from public HostHandle

MIRBuilder:
  still records source meaning only
  does not own object representation or residence
```

## Rejected Designs

```text
reject: drop fallback-only source at constructor boundary
  reason: downstream layers would lose explicit fallback evidence and may infer
          from route/helper context again

reject: treat PublicArrayBoxFallback as direct storage
  reason: it proves public/materialized fallback residence, not raw DirectI64
          or exact ObjectStoragePlan residence

reject: connect handoff directly to backend
  reason: constructor handoff is not a backend consumer contract
```

## Stop Line

```text
do not implement constructor handoff from this row
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
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-SURFACE-001:
  define concrete report fields and open gate for fallback-source constructor
  handoff. Keep implementation closed until the report surface is pinned.
```
