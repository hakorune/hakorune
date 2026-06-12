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

Acceptance:

```text
method call plans derive from registry entries
internal slot and plugin method id remain separate targets
slow dynamic route is explicit
```

### BOXCALL-008

Retire duplicate callable truth.

Acceptance:

```text
PluginLoader no longer owns callable route truth after registration
type_registry is provider / seed source, not parallel planner truth
TypeAbiCatalog remains projection index
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
