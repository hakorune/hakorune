---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-CONSTRUCTION-INVENTORY-001
Scope: Inventory the current ArrayReceiverResidenceSourceConstructor evidence
  before any constructor implementation.
Related:
  - docs/development/current/main/phases/phase-296x/296x-790-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-CONSTRUCTION-SURFACE-001.md
  - docs/development/current/main/phases/phase-296x/296x-789-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-CONSTRUCTION-DESIGN-001.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-CONSTRUCTION-INVENTORY-001

## Purpose

296x-790 fixed the `ArrayReceiverResidenceSourceConstructor` report surface
and open gate. This row fills that surface from current evidence.

The constructor still cannot be implemented: the route, publication, and
materialization inputs exist, but the required `ArrayRepr` or
`ObjectStoragePlan` input is still missing.

## Inventory

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-residence-producer-input-source-construction-inventory-v0
source_evidence=296x-790,296x-789,296x-788,array-repr-ssot,object-storage-plan-boundary-ssot,direct-array-access-plan
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

array_receiver_residence_source_constructor_surface_defined=1
constructor_owner=none
constructor_output=none
constructor_scope=receiver_site_before_length_read
constructor_inputs=RoutePlan|escape_publication_facts|materialization_route|ArrayRepr|ObjectStoragePlan|DirectArrayAccessPlan
constructor_required_input_routeplan=1
constructor_required_input_escape_publication=1
constructor_required_input_materialization_route=1
constructor_required_input_array_repr_or_object_storage=0
constructor_optional_input_direct_array_access_plan=1
constructor_uses_direct_array_access_plan_only=0
constructor_reinterprets_public_arraybox_handle=0
constructor_backend_raw_layout_inference=0
constructor_helper_name_inference=0
constructor_mirbuilder_owner=0
constructor_preserves_public_arraybox_fallback=1
constructor_runtime_execution=0

constructor_candidate_count=1
constructor_eligible_count=0
constructor_rejected_count=1
selected_constructor_candidate_count=0
selected_constructor_candidate_confidence=low
selected_blocker=missing_array_repr_or_object_storage_constructor_input

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
candidate=array_receiver_residence_source_constructor
routeplan_available=1
escape_publication_available=1
materialization_route_available=1
array_repr_available=0
object_storage_plan_available=0
direct_array_access_plan_optional=1
uses_direct_array_access_plan_only=0
eligible=0
reject_reason=missing_array_repr_or_object_storage_constructor_input
```

## Decision

```text
selected_decision=reject_constructor_implementation_until_representation_input_exists
route_proof_available=1
escape_publication_evidence_available=1
materialization_route_available=1
array_repr_or_object_storage_constructor_input_available=0
array_repr_input_available=0
object_storage_plan_input_available=0
direct_array_access_plan_optional_input_available=0
constructor_implementation_allowed=0
backend_direct_handle_bypass_enabled=0
implementation_allowed=0
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-DESIGN-001
```

## Reading

The current optimization path has reached the real upstream gap:

```text
known:
  Array.length route
  publication/fallback evidence
  materialization route

missing:
  a representation source proving ArrayRepr or ObjectStoragePlan at the hot
  receiver before the length read
```

The next row should not add another wrapper around the same absence. It should
design the upstream source that can provide `ArrayRepr` or `ObjectStoragePlan`
evidence to the constructor.

## Stop Line

```text
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
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-DESIGN-001:
  design the upstream representation source that can provide ArrayRepr or
  ObjectStoragePlan evidence for the hot Array receiver. It must remain owned
  by RepresentationPlanner / ObjectStoragePlan / ArrayRepr, not backend raw
  layout inference, helper-name inference, public HostHandle reinterpretation,
  or MIRBuilder object management.
```
