---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-CONSTRUCTION-DESIGN-001
Scope: Design the construction seam for ArrayReceiverResidenceInputSource.
Related:
  - docs/development/current/main/phases/phase-296x/296x-788-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-INVENTORY-001.md
  - docs/development/current/main/phases/phase-296x/296x-787-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-SURFACE-001.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-CONSTRUCTION-DESIGN-001

## Purpose

296x-788 proved that `ArrayReceiverResidenceInputSource` has no eligible
candidate because no `ArrayRepr` or `ObjectStoragePlan` source reaches the hot
receiver before the length read:

```text
input_source_routeplan_available=1
input_source_includes_escape_publication_evidence=1
input_source_includes_materialization_route=1
input_source_includes_array_repr_or_object_storage=0
selected_blocker=missing_array_repr_or_object_storage_source
```

This row designs the construction seam that can eventually build the source.

## Decision

Introduce an analysis-only `ArrayReceiverResidenceSourceConstructor` seam under
representation planning.

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-residence-producer-input-source-construction-design-v0
source_evidence=296x-788,296x-787,array-repr-ssot,object-storage-plan-boundary-ssot,direct-array-access-plan
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h

selected_design=array_receiver_residence_source_constructor
selected_design_confidence=medium
constructor_owner=RepresentationPlanner|ArrayReprSourcePlanner
constructor_output=ArrayReceiverResidenceInputSource
constructor_scope=receiver_site_before_length_read
constructor_inputs=RoutePlan|escape_publication_facts|materialization_route|ArrayRepr|ObjectStoragePlan|DirectArrayAccessPlan
constructor_required_input_routeplan=1
constructor_required_input_escape_publication=1
constructor_required_input_materialization_route=1
constructor_required_input_array_repr_or_object_storage=1
constructor_optional_input_direct_array_access_plan=1
constructor_must_not_use_direct_array_access_plan_only=1
constructor_must_not_reinterpret_public_arraybox_handle=1
constructor_must_not_infer_backend_raw_layout=1
constructor_must_not_use_helper_name=1
constructor_must_not_run_in_mirbuilder=1
constructor_preserves_public_arraybox_fallback=1
constructor_runtime_execution=0
backend_direct_handle_bypass_enabled=0
implementation_allowed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
mirbuilder_object_management_enabled=0
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-CONSTRUCTION-SURFACE-001
summary=ok
```

## Construction Contract

The constructor is not a backend shortcut. It is a join point for existing
facts:

```text
RoutePlan:
  proves the receiver site is Array.length / array_slot_len

escape_publication_facts:
  prove whether public HostHandle publication already happened

materialization_route:
  names public_arraybox_fallback / snapshot / none

ArrayRepr:
  names DirectI64 or PublicArrayBoxFallback when array representation is known

ObjectStoragePlan:
  names exact/native/scalar/generic object representation when object storage
  is known

DirectArrayAccessPlan:
  may provide optional access evidence, but cannot be the only representation
  source
```

The constructor emits either:

```text
ArrayReceiverResidenceInputSource:
  source_confidence=high
  when route, escape/publication, materialization, and ArrayRepr/ObjectStorage
  evidence all agree

none:
  when representation source is absent or only route/access evidence exists
```

## Layer Contract

```text
RepresentationPlanner / ArrayReprSourcePlanner:
  owns source construction
  joins facts
  emits ArrayReceiverResidenceInputSource or none

ArrayRepr / ObjectStoragePlan:
  remain representation truth inputs

DirectArrayAccessPlan:
  optional supporting input only

ArrayReceiverResidenceInput:
  consumes the constructed source later

Backend:
  consumes later proven facts only
  does not construct this source

MIRBuilder:
  records source meaning only
  does not construct this source
```

## Rejected Designs

```text
reject: construct source in backend
  reason: backend consumes representation facts and must not infer raw layout

reject: construct source in MIRBuilder
  reason: MIRBuilder records meaning; it does not own representation residence

reject: construct from helper name
  reason: nyash_array_length_h is an observed hot boundary, not source truth

reject: construct from public HostHandle reinterpretation
  reason: public ArrayBox remains facade/materialization/fallback

reject: construct from DirectArrayAccessPlan alone
  reason: access-plan evidence is not receiver residence proof by itself
```

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
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-CONSTRUCTION-SURFACE-001:
  define concrete report fields and open gate for
  ArrayReceiverResidenceSourceConstructor.
```
