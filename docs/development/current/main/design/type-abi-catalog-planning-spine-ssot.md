---
Status: SSOT
Decision: accepted
Date: 2026-06-13
Scope: TypeAbiCatalog as the thin planning query spine between domain truth,
TypeAbiView adapters, TypeAbiPack snapshots, and domain-owned plans.
Related:
  - docs/development/current/main/design/type-abi-naming-and-box-descriptor-ssot.md
  - docs/development/current/main/design/box-callable-registry-ssot.md
  - docs/development/current/main/design/type-abi-view-and-plan-stamp-ssot.md
  - docs/development/current/main/design/type-abi-box-domain-ssot.md
  - docs/reference/abi/ABI_BOUNDARY_MATRIX.md
  - src/type_abi/mod.rs
  - src/type_abi/pack.rs
---

# Type ABI Catalog Planning Spine SSOT

## Decision

`TypeAbiCatalog` is the thin in-memory query spine for planning and tooling.
It is not central truth and not a generic plan generator.

Naming note: the intended long-term name is `BoxDescriptorCatalog`. The current
`TypeAbiCatalog` name is historical and must not be confused with TypeBox ABI
v2, the plugin execution ABI.

Long-term, Box callable planning moves to `BoxCallableRegistry`. In that final
shape, `TypeAbiCatalog` remains the read-only projection index for Type ABI
and tooling, not the callable registry.

```text
domain truth:
  owned by each domain

TypeAbiView:
  read-only adapter over domain truth

TypeAbiCatalog:
  thin index of published views
  used for cross-domain queries

BoxCallableRegistry:
  canonical Box callable truth for call/new/drop planning

TypeAbiPack:
  generated snapshot for tooling / external consumers

Plan:
  domain-owned execution truth
```

The catalog is the planning entry for cross-domain information. It must not
replace direct access to a domain's own truth.

It also must not own domain truth refresh. Existing refresh owners remain in
place; the catalog is built after those refreshes have produced their normal
metadata.

## Flow

```text
MIR
  -> existing domain refresh pipeline
  -> domain publish views
  -> TypeAbiCatalog
  -> domain planners
  -> domain plans
  -> verifier / lowering
  -> hot path reads Plan only
```

Snapshot flow is separate:

```text
domain truth
  -> TypeAbiView
  -> TypeAbiCatalog
  -> TypeAbiPack
  -> tooling / inspect / optional cursor
```

Do not invert this flow. Planners must not consume `TypeAbiPack`.

## Domain Planner Rule

Domain planners keep direct access to their own truth.

```text
own-domain information:
  read domain truth directly

cross-domain information:
  query TypeAbiCatalog
```

Example:

```rust
impl TypeAbiDomain for FieldDomain {
    fn publish_views(&self, module: &MirModule, catalog: &mut TypeAbiCatalog) {
        for plan in &module.metadata.typed_object_plans {
            catalog.publish(FieldViewAdapter::new(plan));
        }
    }

    fn generate_plans(&self, module: &mut MirModule, catalog: &TypeAbiCatalog) {
        let field_plans = &module.metadata.typed_object_plans;
        let method_views = catalog.query_by_tag(TypeAbiTag::Method);

        // Build domain-owned FieldRoutePlan here.
        let _ = (field_plans, method_views);
    }
}
```

The example is a shape contract, not a requirement that every domain migrate at
once.

## Refresh Boundary

`TypeAbiCatalog` is downstream of existing refresh. It observes refreshed truth;
it does not create, refresh, or canonicalize that truth.

```text
allowed:
  existing refresh owner -> domain truth
  domain truth -> TypeAbiView
  TypeAbiView -> TypeAbiCatalog

forbidden:
  TypeAbiCatalog -> refresh domain truth
  TypeAbiDomain::refresh_truth()
  generic catalog driver owning MIR metadata refresh
```

The compile pipeline shape is:

```rust
fn planning_pipeline(world: &mut CompileWorld) {
    refresh_existing_metadata(world);

    let catalog = TypeAbiCatalog::from_refreshed_world(world.as_readonly());

    for domain in TYPE_ABI_DOMAINS {
        domain.generate_plans(world, &catalog);
    }
}
```

This preserves existing owners such as typed-object plans, fastmem plans,
string corridor plans, plugin route exports, and type registry entries. The
catalog only indexes their published views.

`CompileWorld` in this SSOT is a conceptual boundary name, not a required Rust
type. Do not add a large shared world object just to satisfy this document.
The code-side v0 shape is:

```text
refreshed inputs:
  already refreshed MIR/module metadata
  type_registry method-slot entries
  optional PluginLoader callable snapshot after BoxCallableRegistry projection
  future domain registries only when they already exist

catalog construction:
  TypeAbiCatalog::builder_from_refreshed_world()
  TypeAbiCatalog::from_refreshed_views(...)

forbidden:
  TypeAbiCatalog owning refresh order
  TypeAbiCatalog constructing type_registry / PluginLoader state
  TypeAbiCatalog reading PluginLoader as plugin route truth
  a new global CompileWorld with mutable ownership of every domain
```

Only introduce a real `CompileWorld` / `RefreshedWorld` type if at least two
independent domains need the same read-only input bundle. Until then, pass
the existing domain inputs directly and use the catalog builder as the named
boundary.

Plugin data follows the BoxCallable ownership chain:

```text
PluginLoader
  -> PluginCallableExport
  -> BoxCallableRegistry
  -> TypeAbiCatalog
  -> TypeAbiPack / hako_check / inspect
```

Forbidden inversion:

```text
PluginLoader
  -> TypeAbiCatalog
  -> RoutePlan
```

The detailed plugin task ladder is tracked in
`box-callable-registry-ssot.md` under `PLUGIN-CATALOG-*`.

## Catalog Responsibilities

Allowed:

```text
store stable entry headers
index by tag / id / name
hold view references or opaque domain payload handles
answer cross-domain planning queries
feed TypeAbiPack generation
feed hako_check / inspect summaries
```

Forbidden:

```text
own semantic truth
copy all domain payloads into new descriptor truth
interpret every payload centrally
generate all plans through one generic function
replace domain-owned planners
serve hot execution
serve PlanStamp checks inside hot loops
```

## Pack Boundary

`TypeAbiPack` is downstream of `TypeAbiCatalog`.

```text
allowed:
  TypeAbiCatalog -> TypeAbiPack

forbidden:
  TypeAbiPack -> planner
  TypeAbiPack -> lowering truth
  TypeAbiPack -> hot dispatch
```

Pack is for external/tooling surfaces:

```text
hako_check
inspect bundles
manifest validation
provider capability diagnostics
optional C cursor
```

## TypeAbiDomain Driver

A common driver may exist, but it must stay orchestration-only.

```rust
trait TypeAbiDomain {
    fn publish_views(&self, world: &CompileWorld, catalog: &mut TypeAbiCatalog);
    fn generate_plans(&self, world: &mut CompileWorld, catalog: &TypeAbiCatalog);
    fn verify(&self, world: &CompileWorld, catalog: &TypeAbiCatalog);
    fn report(&self, world: &CompileWorld, out: &mut ReportSink);
}
```

Allowed:

```text
common stage order
common report vocabulary
common catalog creation
reading refreshed truth
```

Forbidden:

```text
refresh_truth hook
ownership of MIR metadata refresh
one giant generate_plans()
central ownership of all domain decisions
forced migration of all existing domains in one slice
```

## Migration Order

### TYPEABI-CATALOG-000

Docs-only catalog planning spine SSOT.

Status: landed 2026-06-13.

Acceptance:

```text
Catalog is planning spine, not truth
Pack is snapshot, not planner input
domain planner reads own truth directly
cross-domain reads go through Catalog
```

### TYPEABI-CATALOG-001

Add in-memory `TypeAbiCatalog` skeleton.

Status: landed 2026-06-13.

Acceptance:

```text
catalog stores headers / indexes only
no TypeAbiPack dependency
no planner consumes pack
type_abi_catalog_hot_lookup_count=0
```

### TYPEABI-METHOD-000

Publish `MethodEntry` views into `TypeAbiCatalog`.

Status: landed 2026-06-13.

Acceptance:

```text
truth_source=type_registry
tag=METHOD_SLOT
catalog query can find method slot headers
MethodEntry remains truth
```

### TYPEABI-BOXDOMAIN-001

Add Box Domain report vocabulary.

Status: landed 2026-06-13.

Acceptance:

```text
box_domain_enabled=1
id_space_mixed_count=0
plugin route truth remains PluginLoader route resolver
```

### BOX-LIFECYCLE-000

Publish plugin lifecycle / method route views through the plugin loader domain.

Acceptance:

```text
LIFECYCLE_ROUTE view exists
PLUGIN_METHOD_ROUTE view exists
route_resolver internals are not broadly public
```

### BOX-LIFECYCLE-001

Define `NewBoxRoutePlan` and `DropBoxRoutePlan` vocabulary.

Acceptance:

```text
birth/fini are LifecycleRoute
NewBox/DropBox use plans
hot path Type ABI lookup remains 0
```

### TYPEABI-CATALOG-CLEAN-000

Docs/report cleanup: keep catalog downstream of existing refresh.

Status: landed 2026-06-13.

Acceptance:

```text
existing refresh pipeline remains owner
TypeAbiDomain has no refresh_truth hook
catalog is built from refreshed world
type_abi_refresh_truth_trait_enabled=0
```

### TYPEABI-CATALOG-CLEAN-001

Add code-side catalog construction vocabulary for refreshed-world entry.

Status: landed 2026-06-13.

Acceptance:

```text
TypeAbiCatalog::builder_from_refreshed_world exists
TypeAbiCatalog::from_refreshed_views exists
catalog construction names the refreshed-world boundary
type_abi_catalog_from_refreshed_world=1
type_abi_catalog_refresh_owner_count=0
TypeAbiCatalog still has no refresh_truth hook
```

### TYPEABI-CATALOG-CLEAN-002

Freeze the minimal refreshed-world input shape.

Status: landed 2026-06-13.

Acceptance:

```text
CompileWorld is documented as conceptual, not required code
v0 catalog inputs are existing refreshed metadata / type_registry / optional PluginLoader snapshot
no new global mutable CompileWorld is introduced
real world type requires two independent domain consumers
```

### PLUGIN-CATALOG bridge

Plugin data may appear in `TypeAbiCatalog` only through a
`BoxCallableRegistry` projection.

Status: planned in `box-callable-registry-ssot.md`.

Acceptance:

```text
PluginLoader -> PluginCallableExport -> BoxCallableRegistry -> TypeAbiCatalog
plugin_loader_to_typeabi_direct_truth_count=0
type_abi_catalog_as_plugin_route_truth_count=0
plugin_catalog_routeplan_consumer_count=0
```

## Report Vocabulary

```text
type_abi_catalog_enabled=1
type_abi_catalog_is_truth=0
type_abi_existing_refresh_preserved=1
type_abi_refresh_truth_trait_enabled=0
type_abi_catalog_from_refreshed_world=1
type_abi_catalog_refresh_owner_count=0
type_abi_catalog_entry_count
type_abi_catalog_query_count
type_abi_catalog_cross_domain_query_count
type_abi_catalog_hot_lookup_count=0

type_abi_pack_from_catalog_count
type_abi_pack_used_by_planner_count=0

domain_planner_own_truth_read_count
domain_planner_catalog_query_count
generic_typeabi_generate_plans_count=0
```

## Final Rule

```text
Catalog is a thin index.
Pack is an artifact.
Truth stays in domains.
Box callable truth moves to BoxCallableRegistry.
Plans execute.
Hot path reads plans only.
```
