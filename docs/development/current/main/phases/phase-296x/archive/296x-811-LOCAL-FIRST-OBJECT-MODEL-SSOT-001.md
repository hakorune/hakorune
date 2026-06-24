---
Status: Landed
Date: 2026-06-16
Task: LOCAL-FIRST-OBJECT-MODEL-SSOT-001
Scope: Reframe the exact-AOT object optimization lane around local-first
  unpublished objects and publication sites.
Related:
  - docs/development/current/main/phases/phase-296x/296x-810-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-FACT-BOUNDARY-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/296x-731-EXACT-OBJECT-PILOT-CLOSEOUT-001.md
  - src/object_storage_plan.rs
---

# LOCAL-FIRST-OBJECT-MODEL-SSOT-001

## Purpose

Stop extending the ArrayReceiver residence proof chain and switch the
optimization model to local-first object planning.

The problem with the current chain is structural:

```text
old direction:
  publish into public Box / HostHandle / Arc world first
  then prove whether backend may recover direct storage later
```

The cleaner direction is:

```text
new direction:
  exact-AOT starts with local unpublished objects
  publish only at explicit public boundaries
```

This preserves source semantics and product runtime behavior while removing the
need to turn fallback evidence into backend proof.

## Decision Report

```text
output_contract=hako-local-first-object-model-ssot-v0
decision=local_first_unpublished_default_for_exact_aot
source_evidence=296x-810,296x-809,296x-731,object_storage_plan_vocabulary
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1

record_source_semantics=value
box_source_semantics=identity_behavior_lifecycle
product_default_runtime_changed=0
product_default_box_world_preserved=1

exact_aot_local_unpublished_default=1
published_object_is_box_world=1
arc_is_post_publication_ownership_form=1
host_handle_is_boundary_representation=1

mirbuilder_object_management_enabled=0
mirbuilder_representation_owner=0
routeplan_call_execution_truth=1
objectplan_representation_truth=1
objectplan_publication_sites_truth=1
standalone_publication_plan_enabled=0
publish_site_detection_is_conservative_escape_analysis=1
unknown_publication_forces_generic_fallback=1
publish_point_precision_requires_ssa_value_boundary=1

array_receiver_residence_chain_extended=0
array_receiver_residence_fact_from_fallback_enabled=0
direct_residence_fact_implementation_selected=0
local_direct_pilot_requires_perf_measurement=1

next_task=OBJECT-PLAN-LOCAL-FIRST-000
summary=ok
```

## Model

Source semantics stay simple:

```text
record:
  identity-free value aggregate

box:
  identity / behavior / lifecycle boundary
```

Exact-AOT representation is local-first:

```text
before publication:
  LocalScalar
  LocalStruct
  LocalIdentityObject

at publication:
  materialize public Box / HostHandle / Arc as required

after publication:
  use generic Box world unless another explicit direct proof exists
```

Product default runtime keeps the generic Box world:

```text
product default:
  Arc / HostHandle / dynamic Box world remains valid
  plugin and runtime compatibility remain valid
```

## Owner Split

Do not add a standalone `PublicationPlan` yet. Keep the first implementation
surface thinner:

```text
ObjectPlan:
  representation choice
  publication sites

RoutePlan:
  call execution route

Backend:
  consumes ObjectPlan + RoutePlan

MIRBuilder:
  emits source meaning only
```

If `ObjectPlan` becomes too large later, publication sites may be extracted
into a separate `PublicationPlan`. That is not the first step.

## Precision Notes

Publish-site detection is still escape analysis. The simplification is not
"analysis disappears"; it is:

```text
old:
  prove a public fallback can be reinterpreted as direct storage

new:
  conservatively detect where a local object must become public
```

The first row should keep this local and conservative:

```text
plugin / extern boundary:
  publish

dynamic Array / Map storage:
  publish

task / Future / Channel boundary:
  publish

dynamic return / unknown receiver:
  publish

unknown:
  generic fallback
```

`direct before publication / generic after publication` is only valid when the
publish point is tied to a precise value boundary. The first pilot should assume
SSA-like value tracking and fail closed if the same local object has ambiguous
publication state.

The model is also not a performance claim. The next pilot must measure whether
the selected pre-publication route actually reduces the product-route body
cost. A correct plan with no measurable win is still a non-keeper.

## New Task Order

```text
OBJECT-PLAN-LOCAL-FIRST-000:
  Define ObjectPlan vocabulary for representation + publication_sites.
  No lowering.

OBJECT-PUBLICATION-INVENTORY-001:
  Inventory object_lifecycle body publication sites.
  Count local candidates and publication reasons.

LOCAL-OBJECT-SHADOW-001:
  Shadow-plan LocalScalar / LocalStruct / LocalIdentityObject / PublishedBox.
  No behavior change.

LOCAL-DIRECT-ARRAY-LEN-PILOT-001:
  If inventory proves pre-publication array length, direct only that route.
  Publish to public ArrayBox only at publication sites.
```

## Stop Line

```text
do not extend ArrayReceiverResidenceInput into fallback Fact
do not implement DirectResidenceFact before local-first inventory
do not create standalone PublicationPlan before ObjectPlan proves too large
do not move object representation ownership into MIRBuilder
do not change product default runtime behavior
do not retire Arc globally
do not retire HostHandle globally
do not infer representation from helper names
do not treat public Box fallback as local direct storage
```

## Meaning

Use this mental model:

```text
Box is semantic.
Arc is post-publication ownership form.
HostHandle is boundary.
Local direct storage is exact-AOT representation.
```

The lane should now look for where an object must become public, not where a
public fallback can be reinterpreted as direct storage.
