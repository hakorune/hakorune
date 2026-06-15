---
Status: SSOT
Decision: accepted
Date: 2026-06-16
Scope: Final compiler object-shape boundary before selfhost MIRBuilder growth.
Related:
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/design/box-callable-registry-ssot.md
  - docs/development/current/main/design/record-box-two-surface-one-substrate-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-824-MIMALLOC-CURRENT-FRONT-OPTIMIZATION-PAUSE-CHECKPOINT-001.md
---

# Compiler Object Final Shape SSOT

## Decision

Before selfhosting more compiler code, keep object semantics and object
representation in separate compiler boxes.

```text
MIRBuilder:
  source meaning emitter

SemanticRefresh / Analysis:
  fact refresh owner

BoxCallableRegistry:
  callable truth

RoutePlan:
  call/new/drop execution truth

ObjectPlan:
  representation + publication-site truth

Backend:
  RoutePlan + ObjectPlan consumer

Runtime:
  generic product Box world
```

Short rule:

```text
MIRBuilder says what the program means.
Plans decide how the program is represented and called.
Backend emits from plans.
Runtime preserves the general fallback world.
```

## Report Vocabulary

```text
compiler_object_final_shape_contract=hako-compiler-object-final-shape-v0
mirbuilder_object_management_enabled=0
mirbuilder_records_object_meaning=1
semantic_refresh_owns_object_facts=1
box_callable_registry_is_callable_truth=1
routeplan_is_call_execution_truth=1
objectplan_is_representation_truth=1
objectplan_is_publication_site_truth=1
standalone_publication_plan_enabled=0
backend_consumes_routeplan_and_objectplan=1
routeplan_objectplan_handoff_contract_defined=1
routeplan_owns_execution_not_representation=1
objectplan_owns_representation_not_execution=1
backend_requires_routeplan_for_direct_call=1
backend_requires_objectplan_for_representation_bypass=1
backend_plan_consumer_guard_enabled=1
backend_plan_consumer_requires_routeplan_and_objectplan=1
backend_existing_flattened_nested_consumer_allowed=1
backend_new_lowering_enabled=0
backend_helper_symbol_inference_enabled=0
backend_method_name_special_case_enabled=0
backend_method_name_special_case_scope=generic_backend_route_inference
guarded_flattened_nested_method_semantic_map_allowed=1
backend_variable_name_special_case_enabled=0
runtime_generic_box_world_preserved=1
product_default_changed=0
selfhost_mirbuilder_metadata_only=1
```

## Layer Contract

### MIRBuilder

Allowed:

```text
NewBox shape
Call shape
FieldGet / FieldSet shape
source span
receiver origin
known type hint
field key
call site id
newbox origin
```

Forbidden:

```text
ObjectStoragePlan construction
ObjectPlan construction
publication site decision
HostHandle bypass decision
Arc retirement decision
stack/native/scalarized representation decision
backend direct-call decision
benchmark/helper/method/variable-name special case
```

### SemanticRefresh / Analysis

Owns refreshed facts, not execution:

```text
known type facts
field layout facts
route facts
escape/publication observations
exactness facts
lifecycle/fini/drop observations
```

### BoxCallableRegistry

Owns callable identity:

```text
box callable key
callable source
callable target
method/lifecycle/property role
```

It does not own storage representation or publication.

### RoutePlan

Owns execution route:

```text
dynamic call
internal slot
user function
intrinsic
plugin invoke
closed-world direct call
new/drop route
```

It does not prove that a receiver can bypass HostHandle or Arc.

### ObjectPlan

Owns representation and publication-site truth:

```text
local unpublished object
published BoxRef
HostHandle escaped
ArcDynBox fallback
exact native struct
exact stack object
scalarized aggregate
flattened nested fields
publication sites
publication reasons
```

For now, publication sites stay inside ObjectPlan:

```text
standalone_publication_plan_enabled=0
```

Standalone `PublicationPlan` is allowed later only if ObjectPlan becomes too
large or publication ordering/dominance needs a separate owner.

### Backend

Consumes plans only:

```text
RoutePlan + ObjectPlan -> direct call / direct field / materialize / fallback
```

Forbidden:

```text
helper symbol inference
method-name special casing
variable-name special casing
benchmark-source branching
direct lowering without RoutePlan
direct lowering without ObjectPlan when representation matters
```

Handoff rule:

```text
RoutePlan alone:
  may prove direct call target
  may not prove receiver storage or publication state

ObjectPlan alone:
  may prove representation / publication state
  may not prove callable target

RoutePlan + ObjectPlan:
  required for C-like exact-AOT lowering that combines direct call with direct
  representation or HostHandle bypass
```

### Runtime

Preserves the product generic world:

```text
generic Box world remains valid
HostHandle remains valid for public boundaries
Arc/shared ownership remains fallback
plugin/runtime diagnostics remain compatible
```

## Selfhost MIRBuilder Contract

The .hako MIRBuilder should grow toward the same boundary:

```text
emit meaning metadata
do not emit representation truth
do not decide publication
do not decide backend routes
```

Required minimum metadata:

```text
source_span
receiver_origin
known_type_hint
field_key
call_site_id
newbox_origin
```

Anything stronger belongs to later facts/plans.

## Stop Lines

```text
do not move object management into MIRBuilder
do not make Type ABI execution truth
do not make hako_check execution truth
do not lower from helper names
do not lower from source variable names
do not globally retire Arc
do not bypass HostHandle without ObjectPlan proof
do not change product default runtime behavior
```

## Next Rows

```text
MIRBUILDER-OBJECT-BOUNDARY-GUARD-001
SELFHOST-MIR-OBJECT-METADATA-001
OBJECTPLAN-PASSIVE-UNIFY-001
ROUTEPLAN-OBJECTPLAN-HANDOFF-001
PUBLICATION-SITE-INVENTORY-GENERIC-001
BACKEND-PLAN-CONSUMER-GUARD-001
COMPILER-OBJECT-SHAPE-CLOSEOUT-001
```
