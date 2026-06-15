---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-CONSTRUCTION-SURFACE-001
Scope: Define the report surface and open gate for
  ArrayReceiverResidenceSourceConstructor before implementation.
Related:
  - docs/development/current/main/phases/phase-296x/296x-789-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-CONSTRUCTION-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/296x-788-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-INVENTORY-001.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-CONSTRUCTION-SURFACE-001

## Purpose

296x-789 selected `ArrayReceiverResidenceSourceConstructor` as an
analysis-only join seam under representation planning. This row defines the
concrete report fields and open gate for that constructor.

It does not implement the constructor, the source, the input, the producer, or
backend direct handle bypass.

## Surface

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-residence-producer-input-source-construction-surface-v0
source_evidence=296x-789,296x-788,array-repr-ssot,object-storage-plan-boundary-ssot,direct-array-access-plan
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

array_receiver_residence_source_constructor_surface_defined=1
constructor_owner=RepresentationPlanner|ArrayReprSourcePlanner|none
constructor_output=ArrayReceiverResidenceInputSource|none
constructor_scope=receiver_site_before_length_read
constructor_inputs=RoutePlan|escape_publication_facts|materialization_route|ArrayRepr|ObjectStoragePlan|DirectArrayAccessPlan
constructor_required_input_routeplan=<0|1>
constructor_required_input_escape_publication=<0|1>
constructor_required_input_materialization_route=<0|1>
constructor_required_input_array_repr_or_object_storage=<0|1>
constructor_optional_input_direct_array_access_plan=1
constructor_uses_direct_array_access_plan_only=0
constructor_reinterprets_public_arraybox_handle=0
constructor_backend_raw_layout_inference=0
constructor_helper_name_inference=0
constructor_mirbuilder_owner=0
constructor_preserves_public_arraybox_fallback=1
constructor_runtime_execution=0

constructor_candidate_count=<n>
constructor_eligible_count=<n>
constructor_rejected_count=<n>
selected_constructor_candidate_count=<n>
selected_constructor_candidate_confidence=low|medium|high
selected_blocker=<blocker|none>

backend_direct_handle_bypass_enabled=0
implementation_allowed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
mirbuilder_object_management_enabled=0
summary=ok
```

## Open Gate

A later constructor implementation row may only open if inventory proves all
of:

```text
constructor_owner=RepresentationPlanner|ArrayReprSourcePlanner
constructor_output=ArrayReceiverResidenceInputSource
constructor_scope=receiver_site_before_length_read
constructor_required_input_routeplan=1
constructor_required_input_escape_publication=1
constructor_required_input_materialization_route=1
constructor_required_input_array_repr_or_object_storage=1
constructor_optional_input_direct_array_access_plan=1
constructor_uses_direct_array_access_plan_only=0
constructor_reinterprets_public_arraybox_handle=0
constructor_backend_raw_layout_inference=0
constructor_helper_name_inference=0
constructor_mirbuilder_owner=0
constructor_preserves_public_arraybox_fallback=1
constructor_runtime_execution=0
constructor_eligible_count>=1
selected_constructor_candidate_confidence=high
```

This gate permits only constructor implementation. It still does not permit
input-source execution, `ArrayReceiverResidenceInput` production,
`ArrayReceiverResidenceFact` production, or backend direct handle bypass.

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
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-CONSTRUCTION-INVENTORY-001:
  fill constructor fields from current route, escape/publication,
  materialization, ArrayRepr, ObjectStoragePlan, and DirectArrayAccessPlan
  evidence. Keep implementation disabled unless a high-confidence constructor
  candidate exists.
```
