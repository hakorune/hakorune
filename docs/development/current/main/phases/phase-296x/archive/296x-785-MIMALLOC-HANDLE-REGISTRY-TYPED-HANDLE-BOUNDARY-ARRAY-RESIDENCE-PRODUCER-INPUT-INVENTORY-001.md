---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-INVENTORY-001
Scope: Inventory the current ArrayReceiverResidenceInput evidence before any
  producer-input implementation.
Related:
  - docs/development/current/main/phases/phase-296x/296x-784-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SURFACE-001.md
  - docs/development/current/main/phases/phase-296x/296x-783-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-DESIGN-001.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-INVENTORY-001

## Purpose

296x-784 fixed the `ArrayReceiverResidenceInput` report surface and open
gate. This row fills that surface from current evidence.

The result is still blocked: route and escape evidence exist, but the hot
receiver does not yet expose DirectArrayAccessPlan, ObjectStoragePlan, or
ArrayRepr evidence that can prove length receiver residence.

## Inventory

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-residence-producer-input-inventory-v0
source_evidence=296x-784,296x-783,296x-782,array-repr-ssot,object-storage-plan-boundary-ssot,direct-array-access-plan
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

array_receiver_residence_input_surface_defined=1
input_owner=none
input_output=none
input_source_routeplan_available=1
input_source_direct_array_access_plan_available=0
input_source_object_storage_plan_available=0
input_source_array_repr_available=0
input_source_escape_facts_available=1
input_can_reference_direct_array_access_plan=1
input_is_direct_array_access_plan_only=0
input_supports_length_receiver_residence=0
input_preserves_public_arraybox_fallback=1
input_public_handle_reinterpretation=0
input_backend_raw_layout_inference=0
input_helper_name_inference=0
input_mirbuilder_owner=0
input_materialization_route_required=1

input_candidate_count=1
input_eligible_count=0
input_rejected_count=1
selected_input_candidate_count=0
selected_input_candidate_confidence=low
selected_blocker=missing_array_receiver_representation_input_source

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
candidate=hot_array_receiver_residence_input
routeplan_available=1
direct_array_access_plan_available=0
object_storage_plan_available=0
array_repr_available=0
escape_facts_available=1
supports_length_receiver_residence=0
eligible=0
reject_reason=missing_array_receiver_representation_input_source
```

## Decision

```text
selected_decision=reject_input_implementation_until_representation_input_source_exists
route_proof_available=1
escape_facts_available=1
array_receiver_residence_input_available=0
direct_array_access_plan_input_available=0
object_storage_plan_input_available=0
array_repr_input_available=0
input_source_required=1
backend_direct_handle_bypass_enabled=0
implementation_allowed=0
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-DESIGN-001
```

## Reading

The current lane has already proved the method route and publication facts, but
it has not yet produced a representation fact for the receiver itself.

That means this row cannot implement `ArrayReceiverResidenceInput`. A later
row must design where the missing source comes from:

```text
RoutePlan:
  tells us the call is Array.length / array_slot_len

escape facts:
  tell us publication/fallback constraints

missing source:
  DirectArrayAccessPlan / ObjectStoragePlan / ArrayRepr evidence that says the
  receiver is direct, native, scalarized, or public fallback before the read
```

## Stop Line

```text
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
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-DESIGN-001:
  design the missing source that can feed ArrayReceiverResidenceInput. The
  source must live in RepresentationPlanner / ObjectStoragePlan / ArrayRepr,
  not MIRBuilder, backend layout inference, helper-name inference, or public
  HostHandle reinterpretation.
```
