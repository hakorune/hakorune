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
  decides generic Box / HostHandle / ArcDynBox / exact native struct /
  scalarized fields / flattened nested fields

ObjectPlan:
  passive owner that pairs ObjectStoragePlan with publication sites
  canonical name for representation + publication-site truth

Backend / exact-AOT lowering:
  emits direct calls, direct fields, native structs, scalar fields, or runtime
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
  Scalarized / FlattenedNestedFields

ObjectPlan:
  owns the passive combination of representation route and publication sites
  does not execute, mutate MIR, or replace RoutePlan

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
  ObjectStoragePlan = ExactNativeStruct / Scalarized / FlattenedNestedFields

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
ExactNativeStruct
Scalarized
FlattenedNestedFields
ArcRemoved
HostHandleRemoved
```

Those are plan/backend outcomes.

## ObjectStoragePlan Vocabulary

Code and design vocabulary are aligned as:

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
    ExactNativeStruct {
        layout_id: LayoutId,
    },
    Scalarized {
        fields: Vec<FieldScalarPlan>,
    },
    FlattenedNestedFields {
        owner_layout_id: LayoutId,
        fields: Vec<FlattenedNestedFieldPlan>,
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
exact_native_struct_candidate_count=<n>
scalarized_object_candidate_count=<n>
flattened_nested_fields_candidate_count=<n>
object_escape_count=<n>
plugin_or_extern_escape_count=<n>
array_or_map_escape_count=<n>
return_escape_count=<n>
selected_object_boundary_owner=<owner|none>
selected_owner_confidence=<low|medium|high>
```

## Historical Pilot: Published Nested Object

The closed `EXACT-OBJECT-PILOT-001` experiment selected
`HakoAllocObjectLifecycleAlignmentResult`, but the candidate is published
through `HakoAllocObjectLifecycleFacade.alignment_result`.  The follow-up
nested publication row proved that the nested handle does not escape and chose
`flatten_nested_fields`.

The experiment required an explicit ObjectStoragePlan consumer for a published
nested object, never a backend benchmark special case. Its final no-keeper
result below supersedes the intermediate implementation queue.

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
  selects valid GenericBox before execution when exact proof is missing

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

## Post-acceptance representation tasks

Status: follow-up plan, not selected. This replaces the historical 715–727
shadow/consumer/retry queue; the pilot's final result is the Closeout Decision
below (route reached, no keeper). Intermediate route-unavailable observations
are not current tasks. Current app selection and `finite_product_goal` stay
unchanged; this plan adds no completion prerequisite to MirBuilder or selfhost.
Reopening requires existing parked-lane gates and explicit owner selection.

Use unchanged `apps/binary-trees/main.hako` as the first candidate, never as a
compiler name predicate. Each task connects the existing semantic plan through
its production consumer; do not create a parallel planner, checker or card.

| Order | Work in existing owners | Completion evidence |
| --- | --- | --- |
| 1. Reproduce | Select matching compiler/runtime artifacts and explicit backend/profile; reuse the fixed smoke. A stale archive is a build-input failure, not a source rejection. | Expected output/exit with no retry, timeout or skip; separate compile/link from execution. Record baseline before optimization. |
| 2. Native representation | ObjectStoragePlan chooses a known layout; allocation, field get/set, call boundaries and destruction consume that same plan. | Unchanged app passes; hot `itemCheck` field access has direct loads, no slot-store lookup, global mutex or runtime field helper, including after inlining. Preserve necessary null/identity/lifetime checks. This does not require arena allocation. |
| 3. Region and reclamation | Extend existing ownership/storage products with recursive result/field region relations and safe reclamation boundaries, only where semantics permit. | Constructor Fault and every exit discharge ownership; no escaped alias or external anchor outlives reset. Reclaim/reuse temporary trees at the actual iteration boundary; live memory does not accumulate with completed iterations. |
| 4. Compare and retain | Reuse performance tooling with matched workload and optimized configurations; inspect generated operations as well as wall time and peak RSS. | Correct output and safety first, then measured time/RSS and baseline ratios. Retain only demonstrated improvements; remove replaced exclusive paths in the same series. No promised Goose/C++/Rust multiplier. |

Region task constraints:

- `make()` returns recursively allocated children to its caller. A local
  noescape bit cannot prove caller/result region membership by itself.
- `iterationCheck()` constructs two trees per loop iteration. A function-wide
  arena that retains all iterations is not the target. `long_lived_tree` is
  last read immediately after construction in this source; its name does not
  prove the same lifetime workload as another benchmark.
- Noescape alone does not allow delayed destruction: preserve fini order,
  Normal/Fault/Return cleanup, partial construction, owning/borrowed fields,
  external resources, weak/lease and observable identity. Start with proved
  eligible objects; do not redefine source lifetime to fit the optimization.
- Contiguous placement, bulk reclamation and 32-bit offsets are independent
  choices. Offset compression additionally needs bounds, alignment, null
  encoding, overflow and growth/reference-stability proofs; it is not required
  for the native-field milestone.
- An optimization proof miss may select an already valid GenericBox plan
  before execution, with a reason. It is not a semantic error or permission
  for retry. A native-only profile reports its unmet requirement explicitly.

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
