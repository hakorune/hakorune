---
Status: SSOT
Decision: accepted
Date: 2026-06-13
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

Acceptance:

```text
box_descriptor_mode=projection_over_box_callable_registry
type_abi_mode remains for compatibility
new reports prefer box_descriptor_* keys
```

### TYPEABI-NAMING-002

Add code aliases without moving files.

Acceptance:

```text
BoxDescriptorView aliases TypeAbiView
BoxDescriptorCatalog aliases TypeAbiCatalog
BoxDescriptorPack aliases TypeAbiPack
no public behavior changes
```

### TYPEABI-NAMING-003

Move modules only after `BoxCallableRegistry` skeleton is stable.

Acceptance:

```text
src/box_descriptor owns descriptor projection code
src/type_abi remains compatibility shim or is retired with docs
tests prove old imports or chosen migration path
```

## Final Rule

```text
TypeBox ABI v2 is the ABI.
BoxCallableRegistry is the truth.
BoxDescriptor is the projection.
RoutePlan is execution.
```
