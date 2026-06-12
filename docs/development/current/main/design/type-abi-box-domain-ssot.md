---
Status: SSOT
Decision: accepted
Date: 2026-06-13
Scope: Box Domain ownership for Type ABI views, plugin route contracts,
NewBox/DropBox planning, and TypeBox slot visibility.
Related:
  - docs/development/current/main/design/type-abi-catalog-planning-spine-ssot.md
  - docs/development/current/main/design/type-abi-view-and-plan-stamp-ssot.md
  - docs/reference/abi/ABI_BOUNDARY_MATRIX.md
  - docs/reference/plugin-abi/nyash_abi_v2.md
  - src/runtime/type_box_abi.rs
  - src/runtime/type_registry.rs
  - src/runtime/plugin_loader_v2/enabled/route_resolver.rs
---

# Type ABI Box Domain SSOT

## Decision

Do not merge TypeBox slots, plugin method ids, birth/fini routes, and Type ABI
snapshots into one giant descriptor truth.

Instead, keep their truth owners separate and make them visible under one
Box Domain.

```text
TypeBox ABI v2:
  external plugin dispatch ABI

type_registry / MethodEntry:
  internal method slot truth

PluginLoader route resolver:
  plugin method / birth / fini route truth

Type ABI:
  read-only view over those truths

TypeAbiCatalog:
  cross-domain planning query spine

Plan:
  execution route for NewBox / DropBox / MethodCall
```

The Box Domain is an ownership and reporting umbrella. It does not collapse id
spaces and does not create a third canonical ABI.

## Canonical ABI Boundary

The external ABI matrix remains unchanged:

```text
canonical ABI surfaces:
  Core C ABI
  TypeBox ABI v2
```

`hako_abi_v1` remains non-canonical draft surface.

Type ABI vNext is a cold descriptor/view surface. It is not a hot execution ABI
and not a replacement for TypeBox ABI v2.

## Box Domain Parts

```text
Box Domain
  MethodSlot
    truth = type_registry / MethodEntry

  PluginMethodRoute
    truth = PluginLoader route resolver / MethodRouteContract

  LifecycleRoute
    truth = PluginLoader route resolver / BirthRouteContract

  InvokeRoute
    truth = PluginLoader route resolver / InvokeRouteContract

  Plans
    NewBoxRoutePlan
    DropBoxRoutePlan
    MethodCallRoutePlan
```

These parts share reporting vocabulary and task placement, but they keep their
own truth source.

## Id Space Rule

Never mix internal TypeBox/type_registry slot ids with plugin method ids.

```text
MethodEntry.slot:
  internal VM/JIT/builtin dispatch slot

MethodRouteContract.method_id:
  plugin TypeBox method id

BirthRouteContract.birth_id:
  plugin lifecycle birth method id

BirthRouteContract.fini_id:
  plugin lifecycle fini method id
```

Required guard vocabulary:

```text
method_slot_id_space=internal_vtable_slot
plugin_method_id_space=plugin_typebox_method_id
id_space_mixed_count=0
```

## Lifecycle Route Rule

Birth/fini are not normal `MethodSlot` entries.

```text
birth:
  LifecycleRoute -> NewBoxRoutePlan

fini:
  LifecycleRoute -> DropBoxRoutePlan
```

Do not model birth/fini as ordinary call slots just because they have method
ids in the plugin ABI. Their execution semantics are object lifecycle routes.

## Plan Boundary

Hot execution consumes plans, not Type ABI views or packs.

```text
NewBox:
  NewBoxRoutePlan

DropBox:
  DropBoxRoutePlan

MethodCall:
  MethodCallRoutePlan
```

Forbidden in hot path:

```text
TypeAbiView lookup
TypeAbiPack query
TypeAbiCursor query
route resolver re-resolution after plan selection
```

Allowed phases:

```text
planning
verification
reporting
hako_check / inspect
debug artifacts
```

## Route Contract Visibility

`MethodRouteContract`, `BirthRouteContract`, and `InvokeRouteContract` may stay
private to `plugin_loader_v2::enabled::route_resolver`.

If Type ABI needs route views, do not make the resolver internals public
directly. Add a narrow exporter inside the plugin loader domain:

```rust
pub(crate) fn export_plugin_route_views(
    loader: &PluginLoaderV2,
    sink: &mut TypeAbiViewSink,
) -> Result<(), TypeAbiError>;
```

The exporter preserves plugin loader ownership and prevents Type ABI from
becoming the route resolver.

## Type ABI Tags

Preferred tag vocabulary:

```text
METHOD_SLOT
PLUGIN_METHOD_ROUTE
LIFECYCLE_ROUTE
INVOKE_ROUTE
FIELD
LAYOUT
MEMORY_ACCESS
STRING_KERNEL
GUI_COMPONENT
PROVIDER
CAPABILITY
```

The current Rust enum may use shorter internal names while the pack schema is
still v0, but report and SSOT vocabulary should keep the distinction explicit.

Payload sources:

```text
METHOD_SLOT:
  source = type_registry::MethodEntry
  payload = name / arity / slot

PLUGIN_METHOD_ROUTE:
  source = MethodRouteContract
  payload = lib_name / type_id / method_id / returns_result

LIFECYCLE_ROUTE:
  source = BirthRouteContract
  payload = type_id / birth_id / fini_id

INVOKE_ROUTE:
  source = InvokeRouteContract
  payload = invoke kind / compat policy
```

## Report Vocabulary

```text
box_domain_enabled=1

method_slot_truth_source=type_registry
plugin_method_route_truth_source=plugin_loader_route_resolver
lifecycle_route_truth_source=plugin_loader_route_resolver
invoke_route_truth_source=plugin_loader_route_resolver

type_abi_mode=view_over_existing_truth
type_abi_pack_is_truth=0
type_abi_hot_lookup_count=0

newbox_route_plan_count
dropbox_route_plan_count
method_call_route_plan_count

newbox_typeabi_hot_lookup_count=0
dropbox_typeabi_hot_lookup_count=0
method_call_typeabi_hot_lookup_count=0

plugin_birth_route_contract_count
plugin_fini_route_contract_count
plugin_method_route_contract_count

method_slot_id_space=internal_vtable_slot
plugin_method_id_space=plugin_typebox_method_id
id_space_mixed_count=0
```

## Task Ladder

### TYPEABI-BOXDOMAIN-000

Docs-only Box Domain SSOT.

Status: landed 2026-06-13.

Acceptance:

```text
MethodSlot / PluginMethodRoute / LifecycleRoute are separate
TypeBox ABI v2 remains external ABI
PluginLoader route resolver remains plugin route truth
Type ABI remains read-only view
```

### TYPEABI-BOXDOMAIN-001

Add report vocabulary for Box Domain route ownership.

Acceptance:

```text
box_domain_enabled=1
method_slot_truth_source=type_registry
plugin_method_route_truth_source=plugin_loader_route_resolver
lifecycle_route_truth_source=plugin_loader_route_resolver
id_space_mixed_count=0
```

### TYPEABI-BOXDOMAIN-002

Add plugin route view exporter inside plugin loader domain.

Acceptance:

```text
route_resolver internals do not become broadly public
Type ABI reads exported views only
PluginLoader remains route truth
```

### TYPEABI-BOXDOMAIN-003

Add `TypeAbiView` adapters for plugin route contracts or exported route views.

Acceptance:

```text
PLUGIN_METHOD_ROUTE view exists
LIFECYCLE_ROUTE view exists
INVOKE_ROUTE view exists if needed
Type ABI pack remains snapshot
```

### TYPEABI-BOXDOMAIN-004

Add `NewBoxRoutePlan` / `DropBoxRoutePlan` vocabulary.

Acceptance:

```text
birth lowers to NewBoxRoutePlan
fini lowers to DropBoxRoutePlan
birth/fini are not treated as MethodSlot
```

### TYPEABI-BOXDOMAIN-005

Cut NewBox / DropBox execution to selected plans when the planner is ready.

Acceptance:

```text
newbox_typeabi_hot_lookup_count=0
dropbox_typeabi_hot_lookup_count=0
route resolver is not re-run inside hot execution
silent fallback after selected lifecycle route=0
```

## Non-Goals

```text
do not redesign TypeBox ABI v2
do not merge internal slots and plugin method ids
do not expose plugin loader internals as generic public API
do not add Type ABI C cursor for this slice
do not make TypeAbiPack planner truth
do not route NewBox/DropBox through Type ABI in hot path
```

## Final Rule

```text
TypeBox is the external plugin dispatch ABI.
PluginLoader is plugin route truth.
Type ABI is the read-only window.
Plan is the execution path.
Birth/fini are LifecycleRoute, not MethodSlot.
```
