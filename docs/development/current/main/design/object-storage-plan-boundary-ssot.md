---
Status: SSOT
Decision: accepted
Date: 2026-06-15
Scope: Exact-AOT object boundary thinning and ObjectStoragePlan ownership.
Related:
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - docs/development/current/main/design/box-callable-registry-ssot.md
  - docs/development/current/main/design/box-object-model-replacement-map-ssot.md
  - docs/development/current/main/design/arc-retirement-and-ownership-substrate-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-709-MIMALLOC-POST-ARRAY-LEN-HELPER-OWNER-REFRESH-001.md
---

# Object Storage Plan Boundary (SSOT)

## Decision

Do not lift Box object management into MIRBuilder.

MIRBuilder may preserve object meaning and route evidence, but object
representation decisions belong to later plans and backend lowering.

```text
mirbuilder_object_management_enabled=0
mirbuilder_records_object_meaning=1
box_callable_registry_is_callable_truth=1
routeplan_is_call_execution_truth=1
object_storage_plan_is_representation_truth=1
backend_consumes_object_storage_plan=1
product_default_runtime_changed=0
```

## Layer Contract

```text
MIRBuilder:
  source semantics -> MIR
  records NewBox / Call / FieldGet / FieldSet meaning
  records receiver origin / known type / source span / candidate metadata
  does not decide object representation

SemanticRefresh / Analysis:
  refreshes escape, ownership, route, layout, and exactness facts

BoxCallableRegistry:
  callable truth
  answers what callable a Box method/lifecycle route resolves to

RoutePlan:
  call / new / drop execution truth
  decides dynamic / direct / plugin / intrinsic / closed-world call routes

ObjectStoragePlan:
  object representation truth
  decides generic Box / HostHandle / ArcDynBox / exact stack object /
  exact native struct / scalarized fields

Backend / exact-AOT lowering:
  emits direct calls, direct fields, native structs, stack objects, or runtime
  calls by consuming RoutePlan and ObjectStoragePlan

Runtime / NyRT:
  keeps the generic product object world
```

Short form:

```text
MIRBuilder makes meaning.
Registry and RoutePlan choose execution routes.
ObjectStoragePlan chooses representation.
Backend lowers to C-like code where proven.
Runtime preserves the general Box world.
```

## Non-Goals

```text
do not make MIRBuilder remove Arc<dyn NyashBox>
do not make MIRBuilder remove HostHandle
do not make MIRBuilder decide stack allocation
do not make MIRBuilder decide scalarization
do not make MIRBuilder decide raw pointer/native layout
do not add benchmark-specific method direct lowering
do not globally retire Arc from optimization evidence
do not change product NyRT default behavior
```

## MIRBuilder Allowed Output

MIRBuilder may improve the material needed by later plans:

```text
NewBox:
  box name / source span / construction origin

Call:
  receiver origin / known receiver type / source span

FieldGet / FieldSet:
  field key / receiver origin / source span

metadata:
  candidate facts that do not decide representation
```

MIRBuilder must not emit a representation choice such as:

```text
ExactStackObject
ExactNativeStruct
Scalarized
ArcRemoved
HostHandleRemoved
```

Those are plan/backend outcomes.

## ObjectStoragePlan Vocabulary

Initial vocabulary may be narrower in code, but the design boundary is:

```rust
enum ObjectStoragePlan {
    GenericBox {
        reason: GenericBoxReason,
    },
    HostHandleEscaped {
        reason: EscapeReason,
    },
    ArcDynBox {
        reason: DynamicReason,
    },
    ExactStackObject {
        layout_id: LayoutId,
    },
    ExactNativeStruct {
        layout_id: LayoutId,
    },
    Scalarized {
        fields: Vec<FieldScalarPlan>,
    },
}
```

`ObjectStoragePlan` is not Type ABI, hako_check, or MIRBuilder truth. It is a
planning artifact consumed by exact-AOT/backend lowering.

## RoutePlan Relationship

`BoxCallableRegistry` and RoutePlan answer the callable side:

```rust
enum MethodCallRoutePlan {
    Dynamic,
    InternalSlot { slot: u16 },
    UserFunction { function_id: FunctionId },
    Intrinsic { intrinsic_id: IntrinsicId },
    PluginInvoke { /* plugin ids */ },
    ClosedWorldDirect {
        function_id: FunctionId,
        receiver_layout: LayoutId,
    },
}
```

Do not combine this with storage representation. A call can be direct while the
receiver remains generic, and a storage plan can be exact while some call route
still falls back.

## Exact Object Candidate Conditions

An exact object plan may be selected only when all required proofs hold:

```text
receiver_type_known=1
constructor_route_known=1
method_route_known=1
field_layout_known=1
plugin_or_extern_escape=0
host_handle_publication_required=0
array_or_map_dynamic_storage_escape=0
return_escape_unplanned=0
dynamic_nyashbox_api_required=0
fini_drop_semantics_closed=1
sync_channel_future_context_boundary_crossed=0
```

If any proof is missing, select `GenericBox` and report the reason. Fallback is
explicit and report-visible; silent fallback is not allowed.

## Product vs Exact-AOT Split

```text
product default NyRT:
  Arc<dyn NyashBox>
  HostHandle
  dynamic Box world
  plugin / diagnostics / compatibility

exact-AOT product route:
  closed-world proof required
  semantics match product route
  may lower proven objects to direct calls/native layout/scalars

exact-AOT diagnostic route:
  measurement floor only
  no product speedup claim
```

## Report Vocabulary

```text
output_contract=hako-object-storage-plan-boundary-v0
mirbuilder_object_management_enabled=0
mirbuilder_records_object_meaning=1
box_callable_registry_is_callable_truth=1
routeplan_is_call_execution_truth=1
object_storage_plan_is_representation_truth=1
backend_consumes_object_storage_plan=1
product_default_runtime_changed=0
```

Inventory rows should additionally report:

```text
arc_dynbox_boundary_count=<n>
host_handle_boundary_count=<n>
dynamic_box_method_route_count=<n>
box_callable_routeplan_dynamic_count=<n>
closed_world_direct_method_candidate_count=<n>
exact_stack_object_candidate_count=<n>
exact_native_struct_candidate_count=<n>
scalarized_object_candidate_count=<n>
object_escape_count=<n>
plugin_or_extern_escape_count=<n>
array_or_map_escape_count=<n>
return_escape_count=<n>
selected_object_boundary_owner=<owner|none>
selected_owner_confidence=<low|medium|high>
```

## Task Order

Use this order after the active array length helper owner refresh.

```text
1. MIMALLOC-POST-ARRAY-LEN-HELPER-OWNER-REFRESH-001
   Classify remaining nyash_array_length_h handle registry / typed handle
   residue after the borrowed-ready keeper.

2. OBJECT-BOUNDARY-INVENTORY-001
   Report Arc / HostHandle / runtime helper / Box method / dynamic route
   boundaries in the object-lifecycle body. No implementation.

3. OBJECT-STORAGE-PLAN-SSOT-001
   Promote this boundary into the code-facing plan vocabulary and guard
   surfaces. Still docs/report only unless a concrete owner is selected.

4. EXACT-OBJECT-PLAN-SHADOW-001
   Generate shadow exact-object candidates without changing execution.
   Report accepted and rejected object storage plans.

5. EXACT-OBJECT-PILOT-001
   If shadow evidence is high confidence, lower one closed-world object front
   through RoutePlan + ObjectStoragePlan. Product default remains unchanged.
```

## Stop Line

```text
do not start ObjectStoragePlan implementation from low-confidence perf residue
do not bypass BoxCallableRegistry / RoutePlan
do not add helper-name or benchmark-name compiler branches
do not move product default runtime toward minimal diagnostic route
do not turn Type ABI / hako_check into execution truth
do not start global Arc retirement from this lane
```
