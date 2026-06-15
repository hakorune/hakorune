---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-INVENTORY-001
Scope: Inventory the current ArrayReceiverRepresentationSource candidates
  before any source implementation.
Related:
  - docs/development/current/main/phases/phase-296x/296x-793-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-SURFACE-001.md
  - docs/development/current/main/phases/phase-296x/296x-792-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-DESIGN-001.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-INVENTORY-001

## Purpose

296x-793 fixed the `ArrayReceiverRepresentationSource` report surface and open
gate. This row fills that surface from current evidence.

The current high-confidence source is fallback-only:
`ArrayRepr::PublicArrayBoxFallback`. It is useful because it gives the
constructor an explicit representation source and prevents backend inference,
but it does not authorize direct handle bypass.

## Inventory

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-inventory-v0
source_evidence=296x-793,296x-792,296x-791,array-repr-ssot,object-storage-plan-boundary-ssot,direct-array-access-plan
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

array_receiver_representation_source_surface_defined=1
representation_source_owner=ArrayRepr
representation_source_output=ArrayReceiverRepresentationSource
representation_source_scope=receiver_site_before_length_read
representation_source_consumed_by=ArrayReceiverResidenceSourceConstructor
representation_source_route_kind=array_slot_len
representation_source_receiver_box_name=ArrayBox
representation_source_array_repr=PublicArrayBoxFallback
representation_source_object_storage_plan_ref=none
representation_source_direct_array_access_plan_ref=none
representation_source_materialization_route=public_arraybox_fallback
representation_source_confidence=high
representation_source_may_provide_array_repr=1
representation_source_may_provide_object_storage_plan=1
representation_source_may_reference_direct_array_access_plan=1
representation_source_is_direct_array_access_plan_only=0
representation_source_preserves_public_arraybox_fallback=1
representation_source_includes_materialization_route=1
representation_source_public_handle_reinterpretation=0
representation_source_backend_raw_layout_inference=0
representation_source_helper_name_inference=0
representation_source_mirbuilder_owner=0

representation_candidate_count=2
representation_eligible_count=1
representation_rejected_count=1
selected_representation_candidate_count=1
selected_representation_candidate_confidence=high
selected_representation_candidate=public_arraybox_fallback_source
selected_blocker=none

direct_representation_candidate_count=1
direct_representation_eligible_count=0
direct_array_repr_available=0
direct_object_storage_plan_available=0
direct_array_access_plan_sufficient=0
direct_representation_selected=0

source_implementation_may_emit_fallback_only=1
source_implementation_must_not_enable_backend_bypass=1
backend_direct_handle_bypass_enabled=0
implementation_allowed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
mirbuilder_object_management_enabled=0
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-IMPLEMENTATION-001
summary=ok
```

## Candidate Table

```text
candidate=public_arraybox_fallback_source
owner=ArrayRepr
array_repr=PublicArrayBoxFallback
object_storage_plan_ref=none
direct_array_access_plan_ref=none
materialization_route=public_arraybox_fallback
preserves_public_arraybox_fallback=1
direct_storage_proof=0
eligible=1
confidence=high

candidate=direct_array_or_object_storage_source
owner=none
array_repr=none
object_storage_plan_ref=none
direct_array_access_plan_ref=none
materialization_route=none
direct_storage_proof=0
eligible=0
confidence=low
reject_reason=missing_direct_arrayrepr_or_object_storage_source
```

## Decision

```text
selected_decision=allow_fallback_representation_source_implementation_only
fallback_representation_source_available=1
fallback_representation_source_confidence=high
direct_representation_source_available=0
direct_backend_bypass_authorized=0
source_implementation_allowed_next=1
backend_direct_handle_bypass_enabled=0
implementation_allowed=0
```

The next implementation row may create a passive
`ArrayReceiverRepresentationSource` for `PublicArrayBoxFallback`. That row must
not claim a speed win or enable direct lowering. It only closes the proof chain
so downstream rows can distinguish:

```text
fallback residence proven:
  public ArrayBox / materialized route is explicit

direct residence not proven:
  backend direct handle bypass remains closed
```

## Stop Line

```text
do not implement ArrayReceiverRepresentationSource from this row
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
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-IMPLEMENTATION-001:
  implement the passive fallback-only ArrayReceiverRepresentationSource.
  It must emit PublicArrayBoxFallback and keep backend direct handle bypass
  disabled.
```
