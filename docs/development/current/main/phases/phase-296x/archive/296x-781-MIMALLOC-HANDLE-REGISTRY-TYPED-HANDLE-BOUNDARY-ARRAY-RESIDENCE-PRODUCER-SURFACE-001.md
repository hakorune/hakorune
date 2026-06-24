---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-SURFACE-001
Scope: Define concrete report fields for the Array receiver residence fact
  producer before any producer implementation.
Related:
  - docs/development/current/main/phases/phase-296x/296x-780-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/296x-779-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-INVENTORY-001.md
  - docs/development/current/main/phases/phase-296x/296x-381-DIRECTARRAY-FAMILY-NEXT-ORDER-TASKBOARD.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-SURFACE-001

## Purpose

296x-780 selected an Array receiver residence fact producer owned by
RepresentationPlanner / ArrayReprFactProducer. This row defines the concrete
report surface a later inventory/implementation row must satisfy.

This row does not implement the producer.

## Decision

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-residence-producer-surface-v0
source_evidence=296x-780,296x-779,directarray-next-order-taskboard,array-repr-ssot,object-storage-plan-boundary-ssot
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h
array_residence_producer_surface_defined=1
producer_owner=RepresentationPlanner|ArrayReprFactProducer
producer_output=ArrayReceiverResidenceFact
producer_input=RoutePlan|DirectArrayAccessPlan|ObjectStoragePlan|escape_facts
producer_order=after_routeplan_and_object_storage_facts_before_backend_lowering
producer_runtime_execution=0
producer_backend_inference=0
producer_mirbuilder_owner=0
producer_helper_name_inference=0
producer_public_handle_reinterpretation=0
producer_preserves_public_arraybox_fallback=1
producer_materialization_route_required=1
backend_direct_handle_bypass_enabled=0
implementation_allowed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
mirbuilder_object_management_enabled=0
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INVENTORY-001
summary=ok
```

## Fact Shape

The producer emits:

```text
ArrayReceiverResidenceFact:
  receiver_site_id=<site>
  route_kind=array_slot_len
  receiver_box_name=ArrayBox
  residence_owner=ObjectStoragePlan|ArrayRepr|none
  residence=direct_array|exact_native_struct|scalarized|public_arraybox_fallback|none
  direct_facts_source=DirectArrayAccessPlan|ArrayRepr|ObjectStoragePlan|none
  direct_facts_proven=<0|1>
  materialization_route_known=<0|1>
  materialization_route=public_arraybox_fallback|snapshot|none
  public_handle_reinterpreted=0
  backend_raw_layout_inference=0
  route_proof_as_storage_proof=0
  host_handle_publication_before_read=<0|1>
  fallback_public_arraybox=1
```

## Inventory Report Fields

The next inventory row must emit:

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-residence-producer-inventory-v0
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

array_residence_producer_surface_defined=1
producer_owner=RepresentationPlanner|ArrayReprFactProducer|none
producer_output=ArrayReceiverResidenceFact|none
producer_input_routeplan_available=<0|1>
producer_input_direct_array_plan_available=<0|1>
producer_input_object_storage_plan_available=<0|1>
producer_input_escape_facts_available=<0|1>
producer_order_valid=<0|1>
producer_runtime_execution=0
producer_backend_inference=0
producer_mirbuilder_owner=0
producer_helper_name_inference=0
producer_public_handle_reinterpretation=0
producer_preserves_public_arraybox_fallback=1
producer_materialization_route_required=1

fact_candidate_count=<n>
fact_eligible_count=<n>
fact_rejected_count=<n>
selected_fact_candidate_count=<n>
selected_fact_candidate_confidence=low|medium|high
selected_blocker=<blocker|none>

backend_direct_handle_bypass_enabled=0
implementation_allowed=0
product_default_changed=0
summary=ok
```

## Open Gate

A later producer implementation row may only open if the producer inventory
proves all of:

```text
producer_owner=RepresentationPlanner|ArrayReprFactProducer
producer_output=ArrayReceiverResidenceFact
producer_input_routeplan_available=1
producer_input_escape_facts_available=1
producer_order_valid=1
producer_runtime_execution=0
producer_backend_inference=0
producer_mirbuilder_owner=0
producer_helper_name_inference=0
producer_public_handle_reinterpretation=0
producer_preserves_public_arraybox_fallback=1
producer_materialization_route_required=1
fact_eligible_count>=1
selected_fact_candidate_confidence=high
```

The implementation row still must not enable backend direct handle bypass. It
may only produce facts for a later residence inventory.

## Stop Line

```text
do not implement the producer from this row
do not implement backend direct handle bypass from this row
do not reinterpret public ArrayBox HostHandle as direct storage
do not infer Rust ArrayBox layout in backend
do not treat array_slot_len route proof as receiver storage proof
do not change nyash.array.birth_h public semantics
do not move Box/Object management into MIRBuilder
do not retire HostHandle globally
do not retire Arc globally
```

## Next

```text
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INVENTORY-001:
  fill the producer report fields from current route/object/direct-array
  evidence, keep implementation disabled, and select no producer implementation
  unless a high-confidence fact producer candidate exists
```
