---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-DESIGN-001
Scope: Design the producer that can supply explicit Array receiver residence
  evidence for the hot Array length site.
Related:
  - docs/development/current/main/phases/phase-296x/296x-779-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-INVENTORY-001.md
  - docs/development/current/main/phases/phase-296x/296x-778-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-SURFACE-001.md
  - docs/development/current/main/phases/phase-296x/296x-381-DIRECTARRAY-FAMILY-NEXT-ORDER-TASKBOARD.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-DESIGN-001

## Purpose

296x-779 rejected backend direct handle bypass because the hot Array receiver
does not carry a selected residence:

```text
array_receiver_residence_owner=none
array_receiver_residence=none
array_receiver_direct_facts_source=none
array_receiver_host_handle_publication_before_read=1
```

This row decides where the missing residence evidence should be produced.

## Decision

Use a narrow Array residence fact producer owned by representation planning /
ArrayRepr, not by MIRBuilder or backend inference.

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-residence-producer-design-v0
source_evidence=296x-779,296x-778,directarray-next-order-taskboard,array-repr-ssot,object-storage-plan-boundary-ssot
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h
selected_design=array_receiver_residence_fact_producer
selected_design_confidence=medium
route_proof_available=1
residence_proof_available=0
producer_owner=RepresentationPlanner|ArrayReprFactProducer
producer_output=ArrayReceiverResidenceFact
producer_input=RoutePlan|DirectArrayAccessPlan|ObjectStoragePlan|escape_facts
producer_must_run_before_backend_lowering=1
producer_must_not_run_in_mirbuilder=1
producer_must_not_run_in_backend_by_layout_inference=1
producer_must_not_use_helper_name=1
producer_must_not_reinterpret_public_arraybox_handle=1
producer_must_preserve_public_arraybox_fallback=1
producer_materialization_route_required=1
backend_direct_handle_bypass_enabled=0
implementation_allowed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
mirbuilder_object_management_enabled=0
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-SURFACE-001
summary=ok
```

## Producer Contract

The producer emits an analysis fact, not executable code:

```text
ArrayReceiverResidenceFact:
  receiver_site_id
  route_kind=array_slot_len
  residence_owner=ObjectStoragePlan|ArrayRepr
  residence=direct_array|exact_native_struct|scalarized|public_arraybox_fallback
  direct_facts_source=DirectArrayAccessPlan|ArrayRepr|ObjectStoragePlan
  direct_facts_proven=<0|1>
  materialization_route=public_arraybox_fallback|snapshot|none
  host_handle_publication_before_read=<0|1>
```

The producer may select a direct residence only when the receiver has an
explicit plan/fact. It must not derive residence from the callable route alone.

## Relationship To DirectArray Rows

The existing DirectArray order remains valid:

```text
DA-SEQ-001: DirectI64 fact inventory
DA-SEQ-002: DirectI64 ArrayRepr producer contract
DA-SEQ-003: DirectI64 ArrayRepr producer implementation
DA-SEQ-004: lowerer consumes ArrayRepr::DirectI64
```

This row does not replace that sequence. It defines the hot mimalloc boundary
consumer-facing proof shape so that the DirectArray producer can later feed it.

## Rejected Designs

```text
reject: MIRBuilder residence producer
  reason: MIRBuilder records source meaning, not representation decisions

reject: backend raw layout inference
  reason: backend consumes residence facts; it does not own Rust ArrayBox layout

reject: helper-name producer
  reason: nyash_array_length_h is a symptom, not storage truth

reject: route proof producer
  reason: array_slot_len proves callable route, not receiver residence

reject: public ArrayBox handle reinterpretation
  reason: public ArrayBox is facade/materialization/fallback, not direct storage
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
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-SURFACE-001:
  define concrete report fields for ArrayReceiverResidenceFact production,
  producer ownership, inputs, outputs, ordering, and rejection reasons
```
