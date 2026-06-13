---
Status: SSOT
Decision: accepted
Date: 2026-06-14
Scope: ARC-RETIRE-005A..005D Box object model replacement map.
Related:
  - docs/development/current/main/design/arc-retirement-and-ownership-substrate-ssot.md
  - docs/development/current/main/design/object-handle-box-identity-contract-ssot.md
  - docs/development/current/main/design/box-callable-registry-ssot.md
  - docs/development/current/main/design/type-abi-catalog-planning-spine-ssot.md
  - docs/development/current/main/workstreams/arc-retirement-current.md
  - src/runtime/box_object_model.rs
  - src/backend/vm_types.rs
---

# Box Object Model Replacement Map (SSOT)

## Decision

ARC-RETIRE-005 defines the replacement map required before any Box family can
retire `Arc<dyn NyashBox>`.

```text
box_object_model_replacement_map=1
clone_share_semantics_classified=1
plugin_lifecycle_owner_defined=1
vmvalue_boxref_carrier_migration_plan=1
typeabi_identity_truth_count=0
arc_hot_path_retirement_started=0
```

This is a contract and reporting slice. It does not replace the runtime object
carrier.

## Scope

```text
ARC-RETIRE-005A:
  clone/share semantics inventory

ARC-RETIRE-005B:
  dyn dispatch / as_any / TypeId surface inventory

ARC-RETIRE-005C:
  plugin lifecycle owner map

ARC-RETIRE-005D:
  VMValue::BoxRef carrier migration plan
```

## Current Runtime Truth

```text
Box object carrier:
  VMValue::BoxRef(Arc<dyn NyashBox>)

Weak object carrier:
  VMValue::WeakBox(Weak<dyn NyashBox>)

Plugin instance carrier:
  PluginBoxV2 -> Arc<PluginHandleInner>

Object identity contract:
  src/runtime/object_identity.rs

Replacement map:
  src/runtime/box_object_model.rs
```

## Replacement Map

### Clone / Share

Current `NyashBox` exposes both value-like cloning and identity-preserving
sharing through the same dyn trait surface.

```text
clone_box:
  may create a fresh value copy
  plugin clone may create a new plugin instance

share_box:
  may preserve stateful identity
  plugin share preserves the plugin instance handle

clone_arc:
  compatibility helper over current Arc carrier
```

Future family gates must preserve these semantics explicitly. A family cannot
retire Arc if clone/share behavior is still only implied by Rust `Arc` or
`dyn NyashBox`.

### Dispatch / Type Identity

Current dispatch and type identity are spread across:

```text
dyn NyashBox
BoxCore
as_any / downcast
type_name
parent_type_id
BoxCallableRegistry
TypeAbiCatalog projection
```

Future dispatch truth is `BoxCallableRegistry` plus per-family route plans.
TypeAbiCatalog remains a read-only projection and never becomes identity truth.

### Plugin Lifecycle

Plugin lifecycle owner is currently:

```text
PluginHandleInner::drop
PluginHandleInner::finalize_now
leak tracker diagnostics
```

Future gates may move storage behind object handles, but the plugin ABI and
fini owner must remain explicit before the carrier changes.

### VMValue Carrier

Current:

```text
VMValue::BoxRef = Arc<dyn NyashBox>
VMValue::WeakBox = Weak<dyn NyashBox>
```

Future:

```text
VMValue::BoxRef = ObjectHandle
VMValue::WeakBox = WeakObjectHandle
```

ARC-RETIRE-005D only exposes this migration plan. It does not alter VMValue
layout.

## Stop Line

```text
do not replace Arc in ARC-RETIRE-005
do not change VMValue::BoxRef layout
do not change plugin ABI
do not make TypeAbiCatalog identity truth
do not claim a family Arc-retirement keeper before ARC-RETIRE-006
```

## Report Vocabulary

```text
box_object_model_replacement_map=1
clone_share_semantics_classified=1
identity_share_box_count_reported=1
clone_returns_fresh_value_count_reported=1
share_preserves_state_count_reported=1
plugin_clone_share_semantics_reported=1
dyn_dispatch_surface_reported=1
downcast_typeid_surface_reported=1
plugin_lifecycle_owner_defined=1
vmvalue_boxref_carrier_migration_plan=1
vmvalue_boxref_current_carrier=arc_dyn_nyashbox
vmvalue_boxref_future_carrier=object_handle
vmvalue_weakbox_current_carrier=weak_dyn_nyashbox
vmvalue_weakbox_future_carrier=weak_object_handle
typeabi_identity_truth_count=0
arc_hot_path_retirement_started=0
```
