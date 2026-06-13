---
Status: SSOT
Decision: accepted
Date: 2026-06-13
Scope: Final Box callable ownership model for builtin boxes, plugin boxes,
user boxes, Type ABI projection, and route plan generation.
Related:
  - docs/development/current/main/design/type-abi-naming-and-box-descriptor-ssot.md
  - docs/development/current/main/design/type-abi-catalog-planning-spine-ssot.md
  - docs/development/current/main/design/type-abi-box-domain-ssot.md
  - docs/development/current/main/design/type-abi-view-and-plan-stamp-ssot.md
  - docs/reference/abi/ABI_BOUNDARY_MATRIX.md
  - docs/reference/plugin-abi/nyash_abi_v2.md
  - src/runtime/type_registry.rs
  - src/runtime/plugin_loader_v2/enabled/route_resolver.rs
  - src/type_abi/catalog.rs
---

# Box Callable Registry SSOT

## Decision

The clean final integration point for plugins and Type ABI is not Type ABI.
It is `BoxCallableRegistry`.

```text
PluginLoader:
  input provider

type_registry:
  builtin input provider

BoxCallableRegistry:
  canonical callable truth

RoutePlan:
  hot execution truth

Type ABI / TypeAbiCatalog / TypeAbiPack:
  read-only projection and artifact surfaces
```

This reduces the long-term shape to four layers:

```text
provider
registry
plan
execution
```

Type ABI remains useful as historical code naming, but the intended long-term
name for that outward window is BoxDescriptor. It is not the center of planning
or execution.

## Ownership

```text
truth:
  BoxCallableRegistry

inputs:
  BuiltinBoxProvider
  PluginBoxProvider
  UserBoxProvider
  IntrinsicProvider

projection:
  BoxDescriptorView
  BoxDescriptorCatalog
  BoxDescriptorPack
  currently implemented as TypeAbiView / TypeAbiCatalog / TypeAbiPack

execution:
  MethodCallRoutePlan
  NewBoxRoutePlan
  DropBoxRoutePlan
```

Plugin and Type ABI become siblings under the registry:

```text
PluginLoader -> BoxCallableRegistry -> RoutePlan
                                |
                                v
                         BoxDescriptor projection
```

## BoxCallable Model

`BoxCallable` is the semantic callable owned by a Box.

```rust
struct BoxCallableKey {
    box_key: BoxKey,
    role: BoxCallableRole,
    name: SymbolId,
    arity: u8,
}
```

Roles:

```rust
enum BoxCallableRole {
    Birth,
    Fini,
    Method,
    StaticMethod,
    PropertyGet,
    PropertySet,
    Operator,
}
```

Targets keep execution and id-space differences explicit:

```rust
enum BoxCallableTarget {
    InternalSlot {
        slot: u16,
    },

    PluginMethod {
        type_id: u32,
        method_id: u32,
        returns_result: bool,
    },

    PluginLifecycle {
        type_id: u32,
        birth_id: Option<u32>,
        fini_id: Option<u32>,
    },

    UserFunction {
        function_id: FunctionId,
    },

    Intrinsic {
        intrinsic_id: IntrinsicId,
    },
}
```

Common key, separated target:

```text
BoxCallableKey:
  shared semantic identity

BoxCallableTarget:
  execution-specific target

slot / method_id / function_id / intrinsic_id:
  never share a raw id space
```

## Provider Roles

`type_registry` is not the final callable registry. It becomes a builtin
provider or seed source.

```text
type_registry / MethodEntry
  -> BuiltinBoxProvider
  -> BoxCallableRegistry target=InternalSlot
```

PluginLoader is not the final route truth. It becomes a plugin metadata reader
and provider.

```text
PluginLoader route resolver contracts
  -> PluginBoxProvider
  -> BoxCallableRegistry target=PluginMethod / PluginLifecycle
```

This removes duplicate route truth while keeping the existing plugin loading
code as the input boundary.

## Descriptor Projection Relationship

BoxDescriptor projects registry state. Current code still uses historical
`TypeAbi*` names for this projection.

```text
BoxCallableRegistry
  -> TypeAbiView
  -> TypeAbiCatalog
  -> TypeAbiPack
```

Allowed:

```text
Type ABI for hako_check / inspect / artifact export
TypeAbiCatalog as read-only projection index
TypeAbiPack as external snapshot
```

Forbidden:

```text
Type ABI as callable truth
TypeAbiPack as planner input
TypeAbiCatalog as execution registry
PluginLoader -> Type ABI -> Plan
```

## Plan Boundary

Route plans are generated from registry entries.

```rust
enum MethodCallRoutePlan {
    InternalSlot {
        slot: u16,
    },
    PluginInvoke {
        type_id: u32,
        method_id: u32,
        returns_result: bool,
        invoke_route: InvokeRoutePlan,
    },
    UserFunction {
        function_id: FunctionId,
    },
    Intrinsic {
        intrinsic_id: IntrinsicId,
    },
    SlowDynamic,
}
```

```rust
enum NewBoxRoutePlan {
    Builtin {
        type_id: u32,
    },
    UserBoxConstructor {
        type_id: u32,
        function_id: FunctionId,
    },
    PluginBirth {
        type_id: u32,
        birth_id: u32,
        fini_id: Option<u32>,
        invoke_route: InvokeRoutePlan,
    },
}
```

```rust
enum DropBoxRoutePlan {
    None,
    UserFini {
        type_id: u32,
        function_id: FunctionId,
    },
    PluginFini {
        type_id: u32,
        fini_id: u32,
        invoke_route: InvokeRoutePlan,
    },
}
```

Hot path reads route plans only.

## Id-Space Guards

Required guard vocabulary:

```text
box_callable_registry_enabled=1
box_callable_common_key_enabled=1

method_slot_id_space=internal_vtable_slot
plugin_method_id_space=plugin_typebox_method_id
lifecycle_id_space=plugin_lifecycle_method_id

id_space_mixed_count=0
slot_compared_to_method_id_count=0
plugin_method_id_used_as_internal_slot_count=0
internal_slot_used_as_plugin_method_id_count=0
```

Truth source vocabulary:

```text
box_callable_truth_source[internal_slot]=type_registry
box_callable_truth_source[plugin_method]=plugin_loader_provider
box_callable_truth_source[lifecycle]=plugin_loader_provider
box_callable_truth_source[user_function]=userbox_metadata
box_callable_truth_source[intrinsic]=intrinsic_registry
```

Projection vocabulary:

```text
type_abi_mode=projection_over_box_callable_registry
type_abi_pack_is_truth=0
type_abi_hot_lookup_count=0
type_abi_catalog_is_execution_registry=0
```

## Migration Plan

### BOXCALL-000

Docs-only BoxCallableRegistry SSOT.

Status: landed 2026-06-13.

Acceptance:

```text
BoxCallableRegistry is final callable truth
Type ABI is projection, not planning truth
PluginLoader becomes provider in the final model
type_registry becomes provider / seed source in the final model
```

### BOXCALL-001

Add `BoxCallableRegistry` skeleton.

Status: landed 2026-06-13.

Acceptance:

```text
registry stores BoxCallableKey -> BoxCallableTarget
id spaces remain typed by target
no execution cutover
no Type ABI dependency
```

### TYPEABI-NAMING-001

Add BoxDescriptor report aliases before broader registry migration.

Status: landed 2026-06-13.

Acceptance:

```text
BoxDescriptor naming exists in reports
TypeAbi* names remain compatibility aliases
TypeBox ABI v2 remains the only plugin execution ABI
```

### TYPEABI-NAMING-002

Add BoxDescriptor code aliases without moving files.

Status: landed 2026-06-13.

Acceptance:

```text
BoxDescriptorView aliases TypeAbiView
BoxDescriptorCatalog aliases TypeAbiCatalog
BoxDescriptorPack aliases TypeAbiPack
no public behavior changes
```

### BOXCALL-002

Seed builtin callables from `type_registry::MethodEntry`.

Status: landed 2026-06-13.

Acceptance:

```text
source=type_registry
target=InternalSlot
MethodEntry remains input truth during migration
```

Code entry:

```text
src/box_callable/providers/builtin_type_registry.rs
```

### BOXCALL-003

Seed plugin callables from PluginLoader route contracts.

Status: landed 2026-06-13.

Acceptance:

```text
source=plugin_loader_provider
target=PluginMethod / PluginLifecycle
route_resolver internals remain encapsulated
```

Code entry:

```text
PluginLoader snapshot:
  src/runtime/plugin_loader_v2/enabled/box_callable_exports.rs
  src/runtime/plugin_loader_v2/enabled/route_resolver.rs

Registry provider:
  src/box_callable/providers/plugin_loader.rs
```

Arity rule:

```text
nyash.toml v2 args -> BoxCallableKey.arity
legacy plugin specs without args -> arity 0 compatibility key
```

### BOXCALL-004

Expose Type ABI projection from `BoxCallableRegistry`.

Status: landed 2026-06-13.

Acceptance:

```text
TypeAbiCatalog can index registry projection
TypeAbiPack can snapshot registry projection
planner does not read TypeAbiPack
```

Code entry:

```text
src/type_abi/box_callable.rs
```

### BOXCALL-005

Add route plan vocabulary.

Status: landed 2026-06-13.

Acceptance:

```text
MethodCallRoutePlan exists
NewBoxRoutePlan exists
DropBoxRoutePlan exists
hot path Type ABI lookup remains 0
```

Code entry:

```text
src/box_callable/route_plan.rs
```

### BOXCALL-006

Cut over NewBox / DropBox planning first.

Status: landed 2026-06-13.

Acceptance:

```text
NewBox/DropBox plans derive from registry entries
PluginLoader is not re-resolved in hot path
fallback after selected lifecycle route=0
```

Code entry:

```text
Semantic route plans:
  src/box_callable/route_plan.rs

Runtime executable lifecycle plans:
  src/runtime/plugin_loader_v2/enabled/lifecycle_route_plan.rs
  src/runtime/plugin_loader_v2/enabled/instance_manager.rs
  src/runtime/plugin_loader_v2/enabled/types.rs
```

Boundary:

```text
BoxCallable RoutePlan:
  semantic ids and route shape only

Plugin lifecycle execution plan:
  runtime-owned fn pointers and compat-shim policy
```

### BOXCALL-007

Cut over method call planning.

Status: landed 2026-06-13.

Acceptance:

```text
method call plans derive from registry entries
internal slot and plugin method id remain separate targets
slow dynamic route is explicit
```

Code entry:

```text
Semantic route plans:
  src/box_callable/route_plan.rs

Runtime executable method plans:
  src/runtime/plugin_loader_v2/enabled/method_route_plan.rs
  src/runtime/plugin_loader_v2/enabled/ffi_bridge.rs
```

### BOXCALL-008

Retire duplicate callable truth.

Status: landed 2026-06-13.

Acceptance:

```text
PluginLoader no longer owns callable route truth after registration
type_registry is provider / seed source, not parallel planner truth
TypeAbiCatalog remains projection index
```

Code entry:

```text
Method resolver cutover:
  src/runtime/plugin_loader_v2/enabled/method_resolver.rs
  src/runtime/plugin_loader_v2/enabled/method_route_plan.rs

Singleton lifecycle cutover:
  src/runtime/plugin_loader_v2/enabled/loader/singletons.rs
  src/runtime/plugin_loader_v2/enabled/lifecycle_route_plan.rs

Report guard rows:
  src/box_callable/report.rs
```

Allowed residual PluginLoader route resolver uses:

```text
provider export:
  PluginLoader reads config/spec/loading metadata and exports PluginCallableExport.

runtime invoke boundary:
  Runtime execution plans close over invoke_box/invoke_shim function pointers.

handle hydration:
  Returned plugin handles may resolve runtime metadata from type_id.
```

Forbidden after this point:

```text
method call / method metadata public helpers must not re-resolve method_id
or returns_result directly from PluginLoader route_resolver.

singleton birth must not bypass BoxCallable lifecycle plans.

TypeAbiCatalog must not become an execution route.
```

### BOXCALL-009

Centralize PluginLoader registry snapshot projection.

Status: landed 2026-06-13.

Decision:

```text
Do not introduce a long-lived registry cache yet.
Do introduce one PluginLoader snapshot entrypoint.
Runtime plans consume that entrypoint instead of seeding providers directly.
```

Acceptance:

```text
plugin_loader_registry_snapshot_entrypoint_count=1
method_plan_direct_provider_seed_count=0
lifecycle_plan_direct_provider_seed_count=0
registry_snapshot_cache_required_count=0
```

Code entry:

```text
Snapshot entry:
  src/runtime/plugin_loader_v2/enabled/box_callable_registry.rs
  src/runtime/plugin_loader_v2/enabled/loader/mod.rs

Consumers:
  src/runtime/plugin_loader_v2/enabled/method_route_plan.rs
  src/runtime/plugin_loader_v2/enabled/lifecycle_route_plan.rs
```

Boundary:

```text
PluginLoader remains provider projection owner.
BoxCallableRegistry remains callable truth snapshot.
Route plans remain execution shape.
No TypeAbiCatalog lookup is introduced in runtime path.
```

### BOXCALL-010

Name runtime invoke binding as a boundary, not route truth.

Status: landed 2026-06-13.

Decision:

```text
Plugin method_id / lifecycle_id route truth lives in BoxCallableRegistry.
Runtime function pointers and compat shim policy live in runtime_invoke_boundary.
Do not expose invoke function pointer lookup as route_resolver truth.
```

Acceptance:

```text
runtime_invoke_boundary_module_count=1
route_resolver_invoke_contract_count=0
runtime_invoke_boundary_derives_fn_pointer_count=1
callable_route_truth_from_invoke_boundary_count=0
```

Code entry:

```text
Runtime invoke boundary:
  src/runtime/plugin_loader_v2/enabled/runtime_invoke_boundary.rs

Consumers:
  src/runtime/plugin_loader_v2/enabled/method_route_plan.rs
  src/runtime/plugin_loader_v2/enabled/lifecycle_route_plan.rs
  src/runtime/plugin_loader_v2/enabled/ffi_bridge.rs
  src/runtime/plugin_loader_v2/enabled/loader/metadata.rs
  src/runtime/plugin_loader_v2/enabled/types.rs
```

Boundary:

```text
runtime_invoke_boundary:
  resolves invoke_box/invoke_shim function pointers and compat shim policy.

route_resolver:
  remains config/spec/provider export helper, not invoke execution route truth.
```

### PLUGIN-CATALOG-000

Document the plugin data projection chain.

Status: planned.

Decision:

```text
PluginLoader data may be exposed through TypeAbiCatalog only after it has been
projected into BoxCallableRegistry.

Allowed:
  PluginLoader -> PluginCallableExport -> BoxCallableRegistry -> TypeAbiCatalog

Forbidden:
  PluginLoader -> TypeAbiCatalog -> RoutePlan
  PluginLoader -> TypeAbiCatalog as callable truth
```

Acceptance:

```text
plugin_catalog_projection_chain_documented=1
plugin_loader_to_typeabi_direct_truth_count=0
type_abi_catalog_as_plugin_route_truth_count=0
```

### PLUGIN-CATALOG-001

Add a registry-to-catalog projection helper for plugin snapshots.

Status: planned.

Scope:

```text
Input:
  BoxCallableRegistry snapshot produced by PluginLoader.

Output:
  TypeAbiCatalog headers for BoxCallable entries.

No direct PluginLoader read from TypeAbiCatalog code.
No RoutePlan generation.
No TypeAbiPack dependency.
```

Acceptance:

```text
plugin_snapshot_catalog_projection_helper_count=1
plugin_snapshot_catalog_entry_count>=0
plugin_snapshot_catalog_reads_loader_directly=0
type_abi_pack_used_by_planner_count=0
```

Candidate code entry:

```text
src/type_abi/box_callable.rs
src/runtime/plugin_loader_v2/enabled/box_callable_registry.rs
```

### PLUGIN-CATALOG-002

Add a hako_check report row for plugin snapshot catalog projection.

Status: planned.

Acceptance:

```text
plugin_loader_registry_snapshot_entrypoint_count=1
plugin_snapshot_catalog_projection_helper_count=1
plugin_loader_to_typeabi_direct_truth_count=0
type_abi_catalog_as_plugin_route_truth_count=0
summary=ok
```

The report remains observation-only. It must not infer route truth from helper
names or Type ABI payload shape.

### PLUGIN-CATALOG-003

Add a unit smoke for empty and non-empty plugin registry catalog projection.

Status: planned.

Acceptance:

```text
empty PluginLoader snapshot can project to an empty TypeAbiCatalog
fixture PluginCallableExport can project to BoxCallableRegistry
BoxCallableRegistry can publish BoxCallable entries to TypeAbiCatalog
Plugin method_id and lifecycle ids remain in BoxCallableTarget only
```

### PLUGIN-CATALOG-004

Decide whether a plugin registry snapshot cache is justified.

Status: planned.

Decision rule:

```text
Do not add a cache by default.
Add a cache only if measurement or repeated-call evidence shows snapshot
construction is a real owner.
```

Acceptance:

```text
registry_snapshot_cache_required_count=0|1
registry_snapshot_cache_default_enabled=0
cache_decision_evidence_path=<path-or-none>
```

### PLUGIN-CATALOG-005

Connect catalog projection to tooling surfaces only.

Status: planned.

Allowed consumers:

```text
hako_check boxcall-contract
inspect/report bundles
TypeAbiPack generation
```

Forbidden consumers:

```text
runtime method invoke
runtime birth/fini
RoutePlan construction
hot path dispatch
```

Acceptance:

```text
plugin_catalog_tooling_consumer_count>=1
plugin_catalog_routeplan_consumer_count=0
plugin_catalog_hot_path_consumer_count=0
```

## Non-Goals

```text
do not redesign TypeBox ABI v2
do not merge slot and method_id spaces
do not route execution through Type ABI
do not force field/string/memory domains into BoxCallableRegistry
do not remove PluginLoader before registry seeding is proven
```

## Final Rule

```text
Plugin is input.
Type ABI is output.
BoxCallableRegistry is the callable truth.
RoutePlan is execution.
```
