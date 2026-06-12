---
Status: SSOT
Decision: accepted
Date: 2026-06-13
Scope: TypeAbiCatalog as the thin planning query spine between domain truth,
TypeAbiView adapters, TypeAbiPack snapshots, and domain-owned plans.
Related:
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

```text
domain truth:
  owned by each domain

TypeAbiView:
  read-only adapter over domain truth

TypeAbiCatalog:
  thin index of published views
  used for cross-domain queries

TypeAbiPack:
  generated snapshot for tooling / external consumers

Plan:
  domain-owned execution truth
```

The catalog is the planning entry for cross-domain information. It must not
replace direct access to a domain's own truth.

## Flow

```text
MIR
  -> domain truth refresh
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
    fn refresh_truth(&self, module: &mut MirModule) {
        refresh_module_typed_object_plans(module);
    }

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
    fn refresh_truth(&self, module: &mut MirModule);
    fn publish_views(&self, module: &MirModule, catalog: &mut TypeAbiCatalog);
    fn generate_plans(&self, module: &mut MirModule, catalog: &TypeAbiCatalog);
    fn verify(&self, module: &MirModule, catalog: &TypeAbiCatalog);
    fn report(&self, module: &MirModule, out: &mut ReportSink);
}
```

Allowed:

```text
common stage order
common report vocabulary
common catalog creation
```

Forbidden:

```text
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

Acceptance:

```text
catalog stores headers / indexes only
no TypeAbiPack dependency
no planner consumes pack
type_abi_catalog_hot_lookup_count=0
```

### TYPEABI-METHOD-000

Publish `MethodEntry` views into `TypeAbiCatalog`.

Acceptance:

```text
truth_source=type_registry
tag=METHOD_SLOT
catalog query can find method slot headers
MethodEntry remains truth
```

### TYPEABI-BOXDOMAIN-001

Add Box Domain report vocabulary.

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

## Report Vocabulary

```text
type_abi_catalog_enabled=1
type_abi_catalog_is_truth=0
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
Plans execute.
Hot path reads plans only.
```
