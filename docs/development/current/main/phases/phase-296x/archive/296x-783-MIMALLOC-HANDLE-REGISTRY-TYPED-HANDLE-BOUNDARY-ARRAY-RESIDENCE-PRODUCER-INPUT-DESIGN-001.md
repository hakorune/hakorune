---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-DESIGN-001
Scope: Design the representation input required before an Array receiver
  residence fact producer can be implemented.
Related:
  - docs/development/current/main/phases/phase-296x/296x-782-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INVENTORY-001.md
  - docs/development/current/main/phases/phase-296x/296x-781-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-SURFACE-001.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-DESIGN-001

## Purpose

296x-782 found that the producer cannot run because the hot receiver has no
representation input:

```text
producer_input_routeplan_available=1
producer_input_direct_array_plan_available=0
producer_input_object_storage_plan_available=0
selected_blocker=missing_direct_array_or_object_storage_input
```

This row decides what input should be produced before the residence fact
producer can be implemented.

## Decision

Introduce a narrow `ArrayReceiverResidenceInput` surface. It is representation
evidence for the receiver, not a backend lowering decision.

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-residence-producer-input-design-v0
source_evidence=296x-782,296x-781,array-repr-ssot,object-storage-plan-boundary-ssot,direct-array-access-plan
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h
selected_design=array_receiver_residence_input_surface
selected_design_confidence=medium
route_proof_available=1
producer_input_available=0
input_owner=RepresentationPlanner|ObjectStoragePlan|ArrayRepr
input_output=ArrayReceiverResidenceInput
input_source=RoutePlan|DirectArrayAccessPlan|ObjectStoragePlan|escape_facts
input_can_reference_direct_array_access_plan=1
input_must_not_be_direct_array_access_plan_only=1
input_must_support_length_receiver_residence=1
input_must_preserve_public_arraybox_fallback=1
input_must_not_reinterpret_public_arraybox_handle=1
input_must_not_infer_backend_raw_layout=1
input_must_not_use_helper_name=1
input_must_not_run_in_mirbuilder=1
backend_direct_handle_bypass_enabled=0
implementation_allowed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
mirbuilder_object_management_enabled=0
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SURFACE-001
summary=ok
```

## Input Contract

The input names whether the receiver already has direct residence evidence:

```text
ArrayReceiverResidenceInput:
  receiver_site_id=<site>
  route_kind=array_slot_len
  receiver_box_name=ArrayBox
  direct_array_plan_available=<0|1>
  object_storage_plan_available=<0|1>
  array_repr_available=<0|1>
  residence_candidate=direct_array|exact_native_struct|scalarized|public_arraybox_fallback|none
  escape_facts_available=<0|1>
  host_handle_publication_before_read=<0|1>
  materialization_route_candidate=public_arraybox_fallback|snapshot|none
```

This input may reference DirectArrayAccessPlan evidence, but it must not be
defined as DirectArrayAccessPlan alone. DirectArrayAccessPlan is an access-plan
seam for load/store sites; the hot length boundary needs receiver residence
evidence before the read.

## Rejected Designs

```text
reject: use DirectArrayAccessPlan alone as the producer input
  reason: DirectArrayAccessPlan is load/store access metadata, not a complete
  receiver residence fact for ArrayBox.length

reject: backend infers ArrayBox layout as input
  reason: backend consumes representation input; it does not own runtime layout

reject: helper-name input
  reason: nyash_array_length_h is a hot symptom, not representation truth

reject: MIRBuilder input owner
  reason: MIRBuilder records source meaning, not representation residence

reject: public ArrayBox handle reinterpretation
  reason: public ArrayBox remains facade/materialization/fallback
```

## Stop Line

```text
do not implement the producer input from this row
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
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SURFACE-001:
  define concrete report fields for ArrayReceiverResidenceInput, including
  owner, input sources, candidate residence, escape/publication evidence, and
  the open gate for a later inventory row
```
