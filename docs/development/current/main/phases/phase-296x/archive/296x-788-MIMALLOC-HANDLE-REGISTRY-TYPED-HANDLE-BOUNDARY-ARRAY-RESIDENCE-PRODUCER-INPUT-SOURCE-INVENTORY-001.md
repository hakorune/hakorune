---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-INVENTORY-001
Scope: Inventory the current ArrayReceiverResidenceInputSource evidence before
  any source implementation.
Related:
  - docs/development/current/main/phases/phase-296x/296x-787-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-SURFACE-001.md
  - docs/development/current/main/phases/phase-296x/296x-786-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-DESIGN-001.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-INVENTORY-001

## Purpose

296x-787 fixed the `ArrayReceiverResidenceInputSource` report surface and
open gate. This row fills that surface from current evidence.

The result remains blocked: the route is known and publication evidence exists,
but no `ArrayRepr` / `ObjectStoragePlan` source reaches the hot receiver before
public `ArrayBox` materialization.

## Inventory

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-residence-producer-input-source-inventory-v0
source_evidence=296x-787,296x-786,296x-785,array-repr-ssot,object-storage-plan-boundary-ssot,direct-array-access-plan
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

array_receiver_residence_input_source_surface_defined=1
input_source_owner=none
input_source_output=none
input_source_scope=receiver_site_before_length_read
input_source_consumed_by=ArrayReceiverResidenceInput
input_source_route_kind=array_slot_len
input_source_receiver_box_name=ArrayBox
input_source_routeplan_available=1
input_source_direct_array_access_plan_ref=none
input_source_object_storage_plan_ref=none
input_source_array_repr=none
input_source_escape_facts_ref=available
input_source_host_handle_publication_before_read=1
input_source_materialization_route=public_arraybox_fallback
input_source_confidence=low

input_source_may_reference_direct_array_access_plan=1
input_source_is_direct_array_access_plan_only=0
input_source_includes_array_repr_or_object_storage=0
input_source_includes_escape_publication_evidence=1
input_source_includes_materialization_route=1
input_source_preserves_public_arraybox_fallback=1
input_source_public_handle_reinterpretation=0
input_source_backend_raw_layout_inference=0
input_source_helper_name_inference=0
input_source_mirbuilder_owner=0

source_candidate_count=1
source_eligible_count=0
source_rejected_count=1
selected_source_candidate_count=0
selected_source_candidate_confidence=low
selected_blocker=missing_array_repr_or_object_storage_source

backend_direct_handle_bypass_enabled=0
implementation_allowed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
mirbuilder_object_management_enabled=0
summary=ok
```

## Candidate Table

```text
candidate=hot_array_receiver_residence_input_source
route_kind=array_slot_len
routeplan_available=1
direct_array_access_plan_ref=none
object_storage_plan_ref=none
array_repr=none
escape_facts_ref=available
host_handle_publication_before_read=1
materialization_route=public_arraybox_fallback
includes_array_repr_or_object_storage=0
eligible=0
reject_reason=missing_array_repr_or_object_storage_source
```

## Decision

```text
selected_decision=reject_input_source_implementation_until_representation_source_exists
route_proof_available=1
escape_publication_evidence_available=1
materialization_route_available=1
array_receiver_residence_input_source_available=0
array_repr_source_available=0
object_storage_plan_source_available=0
direct_array_access_plan_source_available=0
input_source_construction_required=1
backend_direct_handle_bypass_enabled=0
implementation_allowed=0
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-CONSTRUCTION-DESIGN-001
```

## Reading

This row proves that the current hot receiver is still in the public ArrayBox
fallback route before the length read. That is safe, but it is not a direct
residence source.

The missing next piece is not a backend shortcut. It is a construction seam for
the source:

```text
RoutePlan + escape/publication evidence:
  already visible

missing:
  ArrayRepr or ObjectStoragePlan evidence at the receiver site before the read

next:
  design source construction under RepresentationPlanner / ArrayReprSourcePlanner
```

## Stop Line

```text
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
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-CONSTRUCTION-DESIGN-001:
  design the construction seam for ArrayReceiverResidenceInputSource. It must
  join route evidence, escape/publication evidence, materialization route, and
  ArrayRepr/ObjectStoragePlan evidence without backend layout inference or
  public HostHandle reinterpretation.
```
