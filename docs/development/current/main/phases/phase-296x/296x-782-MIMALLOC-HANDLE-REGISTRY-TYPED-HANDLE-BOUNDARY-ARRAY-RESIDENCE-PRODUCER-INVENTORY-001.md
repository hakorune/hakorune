---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INVENTORY-001
Scope: Fill the Array receiver residence producer report fields from current
  route/object/direct-array evidence before any producer implementation.
Related:
  - docs/development/current/main/phases/phase-296x/296x-781-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-SURFACE-001.md
  - docs/development/current/main/phases/phase-296x/296x-780-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/296x-779-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-INVENTORY-001.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INVENTORY-001

## Purpose

296x-781 defined the producer report surface. This row fills it from current
evidence.

This row does not implement the producer.

## Inventory

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-residence-producer-inventory-v0
source_evidence=296x-781,296x-780,296x-779,array-repr-ssot,object-storage-plan-boundary-ssot
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

array_residence_producer_surface_defined=1
producer_owner=none
producer_output=none
producer_input_routeplan_available=1
producer_input_direct_array_plan_available=0
producer_input_object_storage_plan_available=0
producer_input_escape_facts_available=1
producer_order_valid=0
producer_runtime_execution=0
producer_backend_inference=0
producer_mirbuilder_owner=0
producer_helper_name_inference=0
producer_public_handle_reinterpretation=0
producer_preserves_public_arraybox_fallback=1
producer_materialization_route_required=1

fact_candidate_count=1
fact_eligible_count=0
fact_rejected_count=1
selected_fact_candidate_count=0
selected_fact_candidate_confidence=low
selected_blocker=missing_direct_array_or_object_storage_input

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

The producer has one useful input today:

```text
producer_input_routeplan_available=1
```

But it lacks the representation input needed to emit
`ArrayReceiverResidenceFact`:

```text
producer_input_direct_array_plan_available=0
producer_input_object_storage_plan_available=0
producer_owner=none
producer_output=none
```

The current host-handle publication evidence is enough to reject direct
residence, but not enough to produce a direct residence fact. Therefore the
producer implementation remains closed.

## Candidate Table

```text
candidate=hot_array_receiver_residence_fact_producer
routeplan_available=1
direct_array_plan_available=0
object_storage_plan_available=0
escape_facts_available=1
order_valid=0
eligible=0
reject_reason=missing_direct_array_or_object_storage_input
```

## Decision

```text
selected_decision=reject_producer_implementation_until_representation_input_exists
route_proof_available=1
producer_input_available=0
array_receiver_residence_fact_producer_available=0
direct_array_or_object_storage_input_required=1
producer_runtime_execution=0
producer_backend_inference=0
producer_mirbuilder_owner=0
producer_helper_name_inference=0
producer_public_handle_reinterpretation=0
backend_direct_handle_bypass_enabled=0
implementation_allowed=0
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-DESIGN-001
```

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
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-DESIGN-001:
  design the narrow representation input needed by the producer.  The input
  must come from DirectArrayAccessPlan / ObjectStoragePlan / ArrayRepr facts,
  not backend layout inference, helper names, or public ArrayBox handle
  reinterpretation.
```
