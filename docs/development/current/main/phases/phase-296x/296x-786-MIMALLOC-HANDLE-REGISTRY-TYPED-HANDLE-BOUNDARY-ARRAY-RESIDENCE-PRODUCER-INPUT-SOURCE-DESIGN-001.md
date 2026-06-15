---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-DESIGN-001
Scope: Design the missing representation source that can feed
  ArrayReceiverResidenceInput.
Related:
  - docs/development/current/main/phases/phase-296x/296x-785-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-INVENTORY-001.md
  - docs/development/current/main/phases/phase-296x/296x-784-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SURFACE-001.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-DESIGN-001

## Purpose

296x-785 proved that `ArrayReceiverResidenceInput` cannot be implemented from
current evidence:

```text
input_source_routeplan_available=1
input_source_escape_facts_available=1
input_source_direct_array_access_plan_available=0
input_source_object_storage_plan_available=0
input_source_array_repr_available=0
selected_blocker=missing_array_receiver_representation_input_source
```

This row decides where the missing representation source should live.

## Decision

Introduce an `ArrayReceiverResidenceInputSource` produced by representation
planning. It is a pre-input source record for the receiver at the length read
site.

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-residence-producer-input-source-design-v0
source_evidence=296x-785,296x-784,array-repr-ssot,object-storage-plan-boundary-ssot,direct-array-access-plan
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

selected_design=array_receiver_residence_input_source
selected_design_confidence=medium
input_source_owner=RepresentationPlanner|ArrayReprSourcePlanner|ObjectStoragePlan
input_source_output=ArrayReceiverResidenceInputSource
input_source_scope=receiver_site_before_length_read
input_source_consumed_by=ArrayReceiverResidenceInput
input_source_may_reference_direct_array_access_plan=1
input_source_must_not_be_direct_array_access_plan_only=1
input_source_must_include_array_repr_or_object_storage=1
input_source_must_include_escape_publication_evidence=1
input_source_must_include_materialization_route=1
input_source_must_preserve_public_arraybox_fallback=1
input_source_must_not_reinterpret_public_arraybox_handle=1
input_source_must_not_infer_backend_raw_layout=1
input_source_must_not_use_helper_name=1
input_source_must_not_run_in_mirbuilder=1
backend_direct_handle_bypass_enabled=0
implementation_allowed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
mirbuilder_object_management_enabled=0
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-SURFACE-001
summary=ok
```

## Source Contract

The source record answers one question:

```text
At this Array.length receiver site, what representation evidence existed before
public ArrayBox materialization or HostHandle publication?
```

The source shape is:

```text
ArrayReceiverResidenceInputSource:
  receiver_site_id=<site>
  route_kind=array_slot_len
  receiver_box_name=ArrayBox
  routeplan_available=<0|1>
  direct_array_access_plan_ref=<id|none>
  object_storage_plan_ref=<id|none>
  array_repr=DirectI64|PublicArrayBoxFallback|none
  escape_facts_ref=<id|none>
  host_handle_publication_before_read=<0|1>
  materialization_route=public_arraybox_fallback|snapshot|none
  source_confidence=low|medium|high
```

`DirectArrayAccessPlan` may be referenced because it can provide exact
array-access evidence, but it is not sufficient alone. The source must also
carry either `ArrayRepr` or `ObjectStoragePlan` evidence so the later input can
distinguish direct residence from public fallback.

## Layer Contract

```text
RepresentationPlanner / ArrayReprSourcePlanner:
  owns the source record
  joins route facts, direct-array facts, object-storage facts, and escape facts

ObjectStoragePlan:
  remains representation truth when object storage is known

ArrayRepr:
  remains the public ArrayBox / DirectArray bridge

ArrayReceiverResidenceInput:
  consumes the source
  does not re-infer backend layout

Backend:
  consumes later proven facts only
  does not create this source

MIRBuilder:
  records source meaning only
  does not create this source
```

## Rejected Designs

```text
reject: DirectArrayAccessPlan-only source
  reason: access metadata alone is not receiver residence proof

reject: backend raw-layout source
  reason: backend consumes representation source; it does not own runtime layout

reject: helper-name source
  reason: nyash_array_length_h is the symptom, not representation truth

reject: MIRBuilder source owner
  reason: MIRBuilder records meaning, not representation residence

reject: public ArrayBox handle reinterpretation
  reason: public ArrayBox remains facade/materialization/fallback
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
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-SURFACE-001:
  define concrete report fields and open gate for ArrayReceiverResidenceInputSource.
```
