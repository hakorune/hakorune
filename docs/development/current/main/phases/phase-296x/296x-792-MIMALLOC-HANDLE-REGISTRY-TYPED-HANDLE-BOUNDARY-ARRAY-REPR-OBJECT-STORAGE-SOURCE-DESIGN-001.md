---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-DESIGN-001
Scope: Design the upstream representation source required by
  ArrayReceiverResidenceSourceConstructor.
Related:
  - docs/development/current/main/phases/phase-296x/296x-791-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-CONSTRUCTION-INVENTORY-001.md
  - docs/development/current/main/phases/phase-296x/296x-790-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-CONSTRUCTION-SURFACE-001.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-DESIGN-001

## Purpose

296x-791 showed the current upstream gap:

```text
route_proof_available=1
escape_publication_evidence_available=1
materialization_route_available=1
array_repr_or_object_storage_constructor_input_available=0
```

The constructor cannot be implemented until an upstream representation source
provides `ArrayRepr` or `ObjectStoragePlan` evidence for the hot Array receiver
before the length read.

## Decision

Introduce `ArrayReceiverRepresentationSource`. It is the upstream
representation source consumed by `ArrayReceiverResidenceSourceConstructor`.

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-design-v0
source_evidence=296x-791,296x-790,array-repr-ssot,object-storage-plan-boundary-ssot,direct-array-access-plan
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

selected_design=array_receiver_representation_source
selected_design_confidence=medium
representation_source_owner=RepresentationPlanner|ObjectStoragePlan|ArrayRepr
representation_source_output=ArrayReceiverRepresentationSource
representation_source_scope=receiver_site_before_length_read
representation_source_consumed_by=ArrayReceiverResidenceSourceConstructor
representation_source_may_provide_array_repr=1
representation_source_may_provide_object_storage_plan=1
representation_source_may_reference_direct_array_access_plan=1
representation_source_must_not_be_direct_array_access_plan_only=1
representation_source_must_preserve_public_arraybox_fallback=1
representation_source_must_include_materialization_route=1
representation_source_must_not_reinterpret_public_arraybox_handle=1
representation_source_must_not_infer_backend_raw_layout=1
representation_source_must_not_use_helper_name=1
representation_source_must_not_run_in_mirbuilder=1
backend_direct_handle_bypass_enabled=0
implementation_allowed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
mirbuilder_object_management_enabled=0
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-SURFACE-001
summary=ok
```

## Source Contract

`ArrayReceiverRepresentationSource` answers:

```text
What representation proof is available for this Array receiver before public
ArrayBox materialization or HostHandle publication?
```

Shape:

```text
ArrayReceiverRepresentationSource:
  receiver_site_id=<site>
  route_kind=array_slot_len
  receiver_box_name=ArrayBox
  array_repr=DirectI64|PublicArrayBoxFallback|none
  object_storage_plan_ref=<id|none>
  direct_array_access_plan_ref=<id|none>
  materialization_route=public_arraybox_fallback|snapshot|none
  representation_confidence=low|medium|high
```

`array_repr=PublicArrayBoxFallback` is a valid source. It proves fallback
residence, not direct residence. That is still useful because it prevents the
backend from inventing a direct bypass.

`array_repr=DirectI64` or an exact `ObjectStoragePlan` may later prove direct
residence. That direct interpretation belongs to downstream residence input
rows, not this design row.

## Layer Contract

```text
RepresentationPlanner:
  owns source selection and joins representation facts

ArrayRepr:
  remains public ArrayBox / DirectArray bridge

ObjectStoragePlan:
  remains object representation truth

DirectArrayAccessPlan:
  optional supporting evidence only

ArrayReceiverResidenceSourceConstructor:
  consumes ArrayReceiverRepresentationSource

Backend:
  consumes later proven facts only
  does not create this source

MIRBuilder:
  records source meaning only
  does not create this source
```

## Rejected Designs

```text
reject: backend raw-layout source
  reason: backend consumes representation source; it does not own runtime layout

reject: MIRBuilder source owner
  reason: MIRBuilder records meaning, not representation proof

reject: helper-name source
  reason: nyash_array_length_h is an observed hot boundary, not source truth

reject: public ArrayBox handle reinterpretation
  reason: public ArrayBox remains facade/materialization/fallback

reject: DirectArrayAccessPlan-only representation source
  reason: access-plan evidence alone is not representation proof
```

## Stop Line

```text
do not implement the representation source from this row
do not implement the source constructor from this row
do not implement the input source from this row
do not implement the producer input from this row
do not implement the producer from this row
do not implement backend direct handle bypass from this row
do not reinterpret public ArrayBox HostHandle as direct storage
do not infer Rust ArrayBox layout in backend
do not treat DirectArrayAccessPlan alone as receiver residence proof
do not treat array_slot_len route proof as receiver storage proof
do not change nyash.array.birth_h public semantics
do not move Box/Object management into MIRBuilder
do not retire HostHandle globally
do not retire Arc globally
```

## Next

```text
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-SURFACE-001:
  define concrete report fields and open gate for
  ArrayReceiverRepresentationSource.
```
