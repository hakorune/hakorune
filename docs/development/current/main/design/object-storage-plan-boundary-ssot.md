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
  - docs/development/current/main/design/record-box-two-surface-one-substrate-ssot.md
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

`record` and `box` stay separate at the source surface.  This document owns the
object-representation decision layer; the source-language boundary is owned by
`record-box-two-surface-one-substrate-ssot.md`.

Short form:

```text
MIRBuilder makes meaning.
Registry and RoutePlan choose execution routes.
ObjectStoragePlan chooses representation.
Backend lowers to C-like code where proven.
Runtime preserves the general Box world.
```

## Box Boundary Final Placement

The exact-AOT object thinning boundary is not a MIRBuilder feature.  It is a
planning and backend feature.

```text
MIRBuilder:
  owns source-to-MIR meaning only
  may preserve receiver origin / known type / source span / field key evidence
  must not decide Arc removal, HostHandle removal, stack placement, native
  struct layout, or scalarization

BoxCallableRegistry:
  owns callable identity
  answers what Box callable is being invoked

RoutePlan:
  owns execution route
  decides dynamic / direct / plugin / intrinsic / closed-world call shape

ObjectStoragePlan:
  owns representation route
  decides GenericBox / HostHandleEscaped / ArcDynBox / ExactNativeStruct /
  ExactStackObject / Scalarized / FlattenedNestedFields

exact-AOT backend:
  consumes RoutePlan + ObjectStoragePlan
  may lower proven closed-world cases to C-like direct calls and direct fields

Product runtime:
  remains the generic object world
```

This is the required split for C-like lowering:

```text
Source:
  local obj = new Counter()
  obj.inc()
  obj.get()

MIRBuilder:
  NewBox Counter
  Call Counter.inc
  Call Counter.get

Plans:
  NewBoxRoutePlan = closed-world constructor route
  MethodCallRoutePlan = ClosedWorldDirect
  ObjectStoragePlan = ExactNativeStruct / ExactStackObject / Scalarized

Backend:
  struct Counter { value: i64 }
  counter_inc(&mut counter)
  counter_get(&counter)
```

The same source must still have a generic fallback route:

```text
fallback_to_generic_box_supported=1
product_default_changed=0
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

Do not start this lane from an `Arc retirement` goal.  Arc retirement is a
side-lane and must remain per-site/proof-driven:

```text
global_arc_retirement_enabled=0
per_site_arc_elimination_allowed_with_closed_world_proof=1
escaped_or_dynamic_object_uses_generic_route=1
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

## Box / Method / Runtime Boundary Strategy

Do not close the remaining C gap by moving Box management into MIRBuilder.

The C-like path is:

```text
MIRBuilder
  preserves object meaning and route evidence

BoxCallableRegistry
  owns callable identity

RoutePlan
  owns call / new / drop execution route

ObjectStoragePlan
  owns object representation route

exact-AOT backend
  consumes RoutePlan + ObjectStoragePlan and emits direct/native/scalar code
```

The split is intentional:

```text
source semantics:
  stays in MIRBuilder

callable truth:
  stays in BoxCallableRegistry

execution route truth:
  stays in RoutePlan

representation truth:
  stays in ObjectStoragePlan

runtime boundary removal:
  happens in exact-AOT backend only when the plans prove it
```

This prevents three failure modes:

```text
1. MIRBuilder choosing object representation before escape / route / backend
   facts are stable.

2. exact-AOT closed-world decisions leaking into product default NyRT.

3. benchmark/helper/source-name special cases becoming compiler truth.
```

Required report vocabulary for this strategy:

```text
mirbuilder_object_management_enabled=0
mirbuilder_records_object_meaning=1
box_callable_registry_is_callable_truth=1
routeplan_is_call_execution_truth=1
object_storage_plan_is_representation_truth=1
backend_consumes_object_storage_plan=1
closed_world_plan_required=1
fallback_to_generic_box_supported=1
product_default_changed=0
benchmark_name_branch_count=0
helper_name_branch_count=0
```

## Arc / HostHandle Removal Policy

Arc and HostHandle removal is not a global compiler switch.  It is a
per-site exact-AOT backend result that requires closed-world proof.

```text
global_arc_retirement_enabled=0
global_host_handle_retirement_enabled=0
per_site_arc_elimination_allowed_with_closed_world_proof=1
per_site_host_handle_elimination_allowed_with_closed_world_proof=1
escaped_or_dynamic_object_uses_generic_route=1
```

The generic product route keeps the ordinary runtime object boundary:

```text
product route:
  Arc<dyn NyashBox> / object handle remains available
  HostHandle remains available when an object is published across runtime,
  plugin, dynamic collection, reflection, or external boundaries
  dynamic dispatch remains available
```

The exact-AOT route may remove those boundaries only after plans prove the
object is closed:

```text
exact-AOT closed-world route:
  ObjectStoragePlan proves storage representation
  RoutePlan proves method/new/drop execution route
  escape analysis proves no dynamic publication
  backend emits native/direct/scalar code
```

This lane therefore optimizes by shrinking proven exact-AOT boundaries, not by
changing the runtime object model globally.

```text
object_boundary_removal_owner=exact_aot_backend
mirbuilder_object_boundary_removal_owner=0
runtime_generic_object_world_preserved=1
```

## C-Like Lowering Path

The clean path toward C-like output is plan-driven:

```text
1. MIRBuilder records source meaning.
2. SemanticRefresh / analysis refreshes route, layout, ownership, and escape
   facts.
3. BoxCallableRegistry resolves callable identity.
4. RoutePlan selects call/new/drop execution routes.
5. ObjectStoragePlan selects object representation.
6. exact-AOT backend consumes the plans and emits direct/native/scalar code.
7. generic runtime fallback remains available.
```

Do not shortcut this path with benchmark, helper, source, or MIRBuilder
special cases.

```text
source_name_branch_count=0
benchmark_name_branch_count=0
helper_name_branch_count=0
mirbuilder_representation_branch_count=0
closed_world_plan_required=1
generic_fallback_required=1
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

## Current Refinement: Published Nested Object

`EXACT-OBJECT-PILOT-001` selected
`HakoAllocObjectLifecycleAlignmentResult`, but the candidate is published
through `HakoAllocObjectLifecycleFacade.alignment_result`.  The follow-up
nested publication row proved that the nested handle does not escape and chose
`flatten_nested_fields`.

This does not mean the backend may special-case the benchmark.  It means the
next implementation seam must be an explicit ObjectStoragePlan consumer for a
published nested object.

```text
nested_owner=HakoAllocObjectLifecycleFacade.alignment_result
nested_object=HakoAllocObjectLifecycleAlignmentResult
representation_choice=flatten_nested_fields
nested_handle_escape_count=0
mirbuilder_object_management_enabled=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
```

The representation decision remains a plan/backend decision:

```text
MIRBuilder:
  emits Facade.alignment_result field access as source semantics
  does not decide that the nested object is flattened

ObjectStoragePlan:
  records that Facade.alignment_result is represented as nested fields
  owns fallback back to GenericBox when proof is missing

Backend / exact-AOT:
  consumes the plan and lowers field/method access to flattened fields
  never keys behavior by benchmark name, helper name, or source file name

Product runtime:
  keeps the generic handle/object route
```

Do not treat primitive-only fields as sufficient proof.  A primitive-only object
can still be unsafe for exact lowering when it is published through another
object field, returned, stored in a dynamic collection, or passed to plugin /
extern boundaries.

## Flattened Nested Field Pilot Task Order

Use this sequence before claiming `EXACT-OBJECT-PILOT-001` success.

```text
715. EXACT-OBJECT-PILOT-001R
   Retry the pilot only through the selected nested publication plan.  If the
   backend has no flattened-nested-field consumer, close as blocked and select
   the layout seam instead of adding a local patch.

716. EXACT-OBJECT-FLATTENED-NESTED-FIELD-LAYOUT-SSOT-001
   Define how a field such as Facade.alignment_result expands to primitive
   nested fields.  Own field naming, layout ids, fallback, and report
   vocabulary.  No lowering change.

717. EXACT-OBJECT-FLATTENED-NESTED-FIELD-SHADOW-001
   Produce a shadow layout report for the object-lifecycle front:
   flattened_nested_field_count, rewritten_get_candidate_count,
   rewritten_set_candidate_count, and fallback reasons.  No execution change.

718. EXACT-OBJECT-FLATTENED-NESTED-FIELD-BACKEND-SEAM-001
   Add the exact-AOT backend seam that can consume the shadow plan.  This row
   may add passive lowering helpers, but keeps the feature disabled and must
   prove product_default_changed=0.

719. EXACT-OBJECT-PILOT-001S
   Retry the guarded pilot after the backend seam exists.  If field access and
   nested method calls cannot share the same flattened state yet, close as
   blocked and select the state-sharing seam instead of enabling lowering.

720. EXACT-OBJECT-FLATTENED-NESTED-FIELD-STATE-SEAM-001
   Define the backend state-sharing seam so the owner field set, owner field
   gets, and nested method calls all refer to the same flattened nested field
   state.  No field access or method lowering route is enabled here.

721. EXACT-OBJECT-PILOT-001T
   Re-run the pilot preflight after the state seam exists.  This row may select
   the next narrow route-wiring row, but still must not enable backend lowering
   unless field access and method-call consumers are both explicitly routed.

722. EXACT-OBJECT-FLATTENED-NESTED-FIELD-ROUTE-WIRING-001
   Wire field access and direct nested method calls to the passive state seam.
   Keep backend lowering disabled until the guard proves that both read and
   write candidates resolve through the same ObjectStoragePlan state.

723. EXACT-OBJECT-PILOT-001U
   Enable the first guarded exact-object pilot for the selected closed-world
   front only if the state seam and route wiring are green.  Acceptance requires
   no MIRBuilder object management, no benchmark/helper branches, generic
   fallback still available, and product default unchanged.

724. EXACT-OBJECT-PILOT-MEASUREMENT-001
   Measure the pilot.  Claim only exact-AOT product-route evidence.  Do not
   generalize to product NyRT default or global Arc retirement.

725. EXACT-OBJECT-PILOT-EFFECT-ATTRIBUTION-001
   Attribute whether the enabled pilot actually reached generated exact-AOT
   artifacts.  If the route does not appear in generated IR / object evidence,
   do not tune performance; select a backend reachability row.

726. EXACT-OBJECT-FLATTENED-NESTED-FIELD-BACKEND-REACHABILITY-001
   Make the flattened nested field route observable in the backend route used
   by the measured exact-EXE path.  The current evidence says Python
   `src/llvm_py` seams are not the measurement owner; `ny-llvmc`'s default
   boundary driver is.

727. EXACT-OBJECT-FLATTENED-NESTED-FIELD-PLAN-EXPORT-001
   Publish the selected flattened nested ObjectStoragePlan into MIR JSON before
   adding the C ABI boundary consumer.  This prevents the boundary shim from
   inferring representation from Box names, field names, method names, helper
   names, or benchmark names.
```

If 716 or 717 finds a missing proof, keep the pilot blocked.  The correct next
task is another proof row, not a backend shortcut.

## Active Backend Reachability Finding

`EXACT-OBJECT-PILOT-MEASUREMENT-001` did not produce a win:

```text
body_elapsed_ratio_before=112.969
body_elapsed_ratio_after=114.326
winner_claim=0
```

`EXACT-OBJECT-PILOT-EFFECT-ATTRIBUTION-001` then showed that the enabled route
did not reach generated exact-AOT artifacts:

```text
flattened_nested_route_reached=0
generated_ir_contains_synthetic_nested_fields=0
alignment_result.last_requested_count=0
alignment_result.last_normalized_count=0
alignment_result.last_reason_count=0
alignment_result.last_supported_count=0
selected_owner=backend_route_reachability
```

The first reachability attempt updated the Python llvmlite backend seam, but
the measured exact-EXE route uses `ny-llvmc`'s default boundary driver.

```text
python_llvmlite_route_updated=1
measured_exact_exe_driver=ny_llvmc_boundary
python_route_is_measurement_owner=0
boundary_driver_flattened_nested_consumer=0
selected_owner=ny_llvmc_boundary_driver_reachability
```

Therefore the active row must not continue patching Python as the measurement
fix.  The first clean next step is to export the selected ObjectStoragePlan into
MIR JSON:

```text
EXACT-OBJECT-FLATTENED-NESTED-FIELD-PLAN-EXPORT-001:
  object_storage_plan_mir_json_export_enabled=1
  backend_lowering_enabled=0
  boundary_driver_flattened_nested_consumer=0
```

Only after that export exists may the boundary driver consume it:

```text
EXACT-OBJECT-FLATTENED-NESTED-FIELD-BOUNDARY-CONSUMER-001:
  boundary_driver_flattened_nested_consumer=1
  uses_object_storage_plan_metadata=1
  benchmark_name_branch_count=0
  helper_name_branch_count=0
```

## Current Task Queue

The active exact-object lane has passed the representation-publication stage.
Do not treat the previous Python backend seam as the measured owner; the
measured exact-EXE route is the `ny-llvmc` boundary C ABI shim.

```text
1. EXACT-OBJECT-FLATTENED-NESTED-FIELD-PLAN-EXPORT-001
   Landed.  ObjectStoragePlan metadata is exported into MIR JSON as a read-only
   plan surface:
     object_storage_plan_mir_json_export_enabled=1
     backend_lowering_enabled=0
     boundary_driver_flattened_nested_consumer=0
     mirbuilder_object_management_enabled=0

2. EXACT-OBJECT-FLATTENED-NESTED-FIELD-BOUNDARY-CONSUMER-001
   Active.
   Teach the ny-llvmc boundary C ABI shim to consume the exported plan.
   The consumer must key from ObjectStoragePlan metadata, not from benchmark,
   helper, or source-file names.

   Required work:
     field_access_lowering_connected=1
     nested_method_lowering_connected=1
     generated_artifact_reachability_proven=1

   Allowed method-name use:
     only after the receiver is proven to be the ObjectStoragePlan-provided
     flattened nested view.  Method names may select the semantic nested view
     operation, but must not infer that the object is flattened.

3. EXACT-OBJECT-PILOT-001V
   Retry the guarded exact-object pilot after the measured boundary route can
   consume the plan.  This row may claim reachability only if generated
   artifacts contain the flattened nested field route.

4. EXACT-OBJECT-PILOT-MEASUREMENT-002
   Landed.  The product exact-AOT route was measured after reachability was
   proven:
     body_elapsed_ratio_before=114.326
     body_elapsed_ratio_after=117.038
     winner_claim=0

5. EXACT-OBJECT-PILOT-CLOSEOUT-001
   Landed.  Close the first exact-object pilot as a no-keeper boundary
   experiment:
     object_storage_plan_route_reached=1
     keeper_claim=0
     global_arc_retirement_claim=0
     global_host_handle_retirement_claim=0
     mirbuilder_object_management_enabled=0
     selected_next=MIMALLOC-BODY-TIMING-FRESH-OWNER-SELECTION-002
```

If task 2 cannot consume the exported plan without adding representation
decisions to MIRBuilder, stop the lane and update this SSOT.  The correct fix is
another backend plan-consumer seam, not MIRBuilder object management.

A diagnostic route switch to Python remains possible, but it must be explicit
and report-visible:

```text
exact_object_pilot_route=python_llvmlite_diagnostic
route_switch_explicit=1
product_default_changed=0
winner_claim=0
```

## Stop Line

```text
do not start ObjectStoragePlan implementation from low-confidence perf residue
do not bypass BoxCallableRegistry / RoutePlan
do not add helper-name or benchmark-name compiler branches
do not move product default runtime toward minimal diagnostic route
do not turn Type ABI / hako_check into execution truth
do not start global Arc retirement from this lane
do not treat method names as proof of flattened representation
```

## Closeout Decision

The first exact-object pilot is closed as a no-keeper boundary experiment.

```text
output_contract=hako-exact-object-pilot-closeout-v0
source_evidence=296x-730
target_front=object_lifecycle_body
object_storage_plan_route_reached=1
pilot_exact_object_enabled=1
body_elapsed_ratio_before=114.326
body_elapsed_ratio_after=117.038
winner_claim=0
keeper_claim=0
global_arc_retirement_claim=0
global_host_handle_retirement_claim=0
product_default_changed=0
mirbuilder_object_management_enabled=0
benchmark_name_branch_count=0
helper_name_branch_count=0
type_abi_execution_truth=0
selected_next=MIMALLOC-BODY-TIMING-FRESH-OWNER-SELECTION-002
summary=ok
```

This is the important architectural result:

```text
ObjectStoragePlan is the right layer for object representation decisions.
The exact-AOT backend is the right layer for C-like lowering.
MIRBuilder is still not the owner of Box object management.
No global Arc / HostHandle retirement follows from this pilot.
```

The next optimization row must return to owner-first evidence.  Do not continue
editing ObjectStoragePlan, C ABI shims, or runtime object representation unless
a fresh high-confidence owner selects that boundary again.
