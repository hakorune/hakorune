---
Status: SSOT
Decision: accepted
Date: 2026-08-07
Scope: Naming boundary between TypeBox ABI v2, historical TypeAbi* code,
BoxDescriptor projection surfaces, and BoxCallableRegistry.
Related:
  - docs/reference/abi/ABI_BOUNDARY_MATRIX.md
  - docs/reference/plugin-abi/nyash_abi_v2.md
  - docs/development/current/main/design/box-callable-registry-ssot.md
  - docs/development/current/main/design/type-abi-view-and-plan-stamp-ssot.md
  - docs/development/current/main/design/type-abi-catalog-planning-spine-ssot.md
  - docs/development/current/main/design/type-abi-box-domain-ssot.md
  - src/type_abi/mod.rs
---

# Type ABI Naming And BoxDescriptor SSOT

## Decision

The name `Type ABI` is overloaded and must be clarified before more
implementation work.

```text
TypeBox ABI v2:
  real external plugin execution ABI

runtime::type_box_abi::TypeBox:
  older Tier-0 external ABI skeleton; not runtime type metadata

core::type_box::TypeBox:
  in-process runtime type metadata; not an external execution ABI

MirBuilder TypeRegistry:
  ValueId -> MirType/origin facts; not a Box descriptor or plugin ABI

TypeAbiView / TypeAbiCatalog / TypeAbiPack:
  historical code names for descriptor projection surfaces

BoxCallableRegistry:
  final internal callable truth

BoxDescriptorView / BoxDescriptorCatalog / BoxDescriptorPack:
  intended future names for descriptor projection surfaces
```

The current `TypeAbi*` code is not an execution ABI. It is a read-only
descriptor/projection surface.

## Correct Naming

Use these meanings in docs and reports:

```text
TypeBox ABI v2:
  plugin / DLL C ABI
  external execution boundary
  resolve + invoke_id path

TypedFast exact-entry:
  verified Provider Box binding profile
  not a third semantic API or descriptor authority

BoxCallableRegistry:
  Rust internal callable truth
  source for MethodCall / NewBox / DropBox route plans

BoxDescriptorView:
  read-only view over registry or domain truth

BoxDescriptorCatalog:
  thin in-memory descriptor index

BoxDescriptorPack:
  tooling / inspect / optional cursor snapshot
```

Historical names still present in code:

```text
TypeAbiView:
  historical name for BoxDescriptorView

TypeAbiCatalog:
  historical name for BoxDescriptorCatalog

TypeAbiPack:
  historical name for BoxDescriptorPack

runtime::type_box_abi::TypeBox:
  historical Tier-0 ABI skeleton name

core::type_box::TypeBox:
  runtime metadata name; future RuntimeTypeDescriptor candidate

src/mir/builder/type_registry.rs TypeRegistry:
  MIR value type facts; future MirValueTypeFacts candidate
```

## Forbidden Readings

Do not describe `TypeAbiView`, `TypeAbiCatalog`, or `TypeAbiPack` as:

```text
external plugin execution ABI
common C API for DLL dispatch
canonical callable truth
route planner truth
hot path dispatch surface
```

Those meanings belong to:

```text
external plugin execution ABI:
  TypeBox ABI v2

callable truth:
  BoxCallableRegistry

hot execution:
  RoutePlan
```

## Migration Rule

Do not perform a large rename before the registry direction is stable.

Allowed now:

```text
docs clarify historical names
report vocabulary introduces BoxDescriptor terms
new modules may use BoxDescriptor naming
compat re-export may keep TypeAbi* names temporarily
```

Delayed:

```text
rename src/type_abi to src/box_descriptor
rename TypeAbiView to BoxDescriptorView
rename TypeAbiCatalog to BoxDescriptorCatalog
rename TypeAbiPack to BoxDescriptorPack
rename report keys that external scripts already consume
```

## Task Ladder

### TYPEABI-NAMING-000

Docs-only naming clarification.

Status: landed 2026-06-13.

Acceptance:

```text
TypeBox ABI v2 is the plugin execution ABI
TypeAbi* is marked as descriptor projection historical naming
BoxDescriptor* future naming is defined
BoxCallableRegistry remains callable truth
```

### TYPEABI-NAMING-001

Add report aliases for BoxDescriptor naming.

Status: landed 2026-06-13.

Acceptance:

```text
box_descriptor_mode=projection_over_box_callable_registry
type_abi_mode remains for compatibility
new reports prefer box_descriptor_* keys
```

### TYPEABI-NAMING-002

Add code aliases without moving files.

Status: landed 2026-06-13.

Acceptance:

```text
BoxDescriptorView aliases TypeAbiView
BoxDescriptorCatalog aliases TypeAbiCatalog
BoxDescriptorPack aliases TypeAbiPack
no public behavior changes
```

### TYPEABI-NAMING-003

Extend BoxDescriptor naming aliases to registry projection helpers.

Status: landed 2026-06-13.

Acceptance:

```text
BoxDescriptorBoxCallableView aliases BoxCallableEntryView
publish_box_callable_descriptors aliases publish_box_callable_registry
build_box_descriptor_catalog_from_callable_registry aliases registry catalog projection
build_box_descriptor_callable_pack aliases registry pack projection
no file moves
no public behavior changes
```

### TYPEABI-NAMING-004

Move modules only after descriptor naming is stable in consumers.

Acceptance:

```text
four-surface census includes TypeBox ABI v2, runtime ABI skeleton TypeBox,
  runtime metadata TypeBox, and MirBuilder TypeRegistry
src/box_descriptor owns descriptor projection code
src/type_abi remains compatibility shim or is retired with docs
tests prove old imports or chosen migration path
implemented renames update the affected docs/reference pages in the same slice
```

## Final Rule

```text
ProviderSlot contract is API meaning.
TypeBox ABI v2 is the external TLV execution boundary.
TypedFast is an exact-entry Provider Box binding profile, not a third truth.
BoxCallableRegistry is admitted callable and selected-target truth.
BoxDescriptor is the projection.
RoutePlan is semantic execution choice.
RuntimeExecutablePlan is exact physical execution binding.
```
