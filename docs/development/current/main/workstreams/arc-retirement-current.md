---
Status: Taskboard
Date: 2026-06-14
Scope: Arc retirement / ownership substrate side-lane task order.
Related:
  - docs/development/current/main/design/arc-retirement-and-ownership-substrate-ssot.md
  - docs/development/current/main/design/runtime-gc-policy-and-order-ssot.md
  - docs/development/current/main/design/box-callable-registry-ssot.md
  - docs/development/current/main/design/type-abi-catalog-planning-spine-ssot.md
---

# Arc Retirement Current Taskboard

This is a side-lane taskboard. It does not change the active exact-AOT
optimization lane.

## Current Decision

```text
arc_retirement_mode=tasking_only
arc_hot_path_retirement_started=0
active_optimization_lane_changed=0
```

Arc retirement is allowed as docs/inventory planning now. Implementation starts
only after a concrete retirement seam has its own gate.

## Task Order

### ARC-RETIRE-000: SSOT and taskboard

Status:

```text
landed_by=
  docs/development/current/main/design/arc-retirement-and-ownership-substrate-ssot.md
  docs/development/current/main/workstreams/arc-retirement-current.md
```

Acceptance:

```text
arc_is_current_reclamation_owner=1
gc_is_diagnostic_only=1
arc_role_count=4
box_object_model_replacement_required=1
arc_hot_path_retirement_started=0
summary=ok
```

### ARC-RETIRE-001: Arc role inventory

Read-only inventory.

```text
count:
  Arc<dyn NyashBox>
  Weak<dyn NyashBox>
  Box<dyn NyashBox>
  Arc<PluginHandleInner>
  Arc<Mutex<dyn NyashBox>>
  Any / TypeId / parent_type_id
  clone_box / share_box / clone_arc

classify:
  hot path
  runtime substrate
  plugin boundary
  compatibility
  diagnostics
```

Output:

```text
arc_role_inventory_count
arc_identity_truth_count
arc_plugin_boundary_count
arc_hot_path_count
arc_compat_count
```

### ARC-RETIRE-002: RC MIR coverage inventory

Read-only inventory plus gap report.

```text
classify:
  default rc_insertion pass behavior
  rc-insertion-minimal feature behavior
  builder direct ReleaseStrong emission
  ReleaseStrong backend support
  retain support
  no-op backend surfaces
```

Acceptance:

```text
rc_insertion_single_source_reported=1
default_rc_insertion_active_reported=1
retain_strong_gap_reported=1
backend_noop_surface_reported=1
```

### ARC-RETIRE-003: ObjectHandle / BoxIdentity contract

Docs-only design before code.

```text
define:
  ObjectHandle
  generation
  weak handle
  root visibility
  plugin instance mapping
  builtin identity
  scope/fini ownership
```

Acceptance:

```text
object_handle_contract_defined=1
typeabi_identity_truth_count=0
box_callable_dispatch_truth_count>0
```

### ARC-RETIRE-004: Host handle table seam

First possible implementation seam, but only after ARC-RETIRE-003.

```text
goal:
  keep external u64 handle ABI stable
  replace/classify backing Arc table responsibilities
  preserve borrowed access APIs
```

Non-goals:

```text
no plugin ABI change
no Box trait rewrite
no global Arc removal claim
```

### ARC-RETIRE-005: Box object model replacement map

Docs + inventory.

```text
map:
  dyn dispatch
  clone_box
  share_box
  downcast / TypeId
  type_name
  Send / Sync
  finalization
  plugin lifecycle
```

Acceptance:

```text
box_object_model_replacement_map=1
clone_share_semantics_classified=1
plugin_lifecycle_owner_defined=1
```

### ARC-RETIRE-006: Family retirement gate

Define the gate for retiring Arc in one Box family.

```text
required:
  object_identity_owner_exists=1
  refcount_storage_owner_exists=1
  atomic_free_on_zero_exists=1
  dispatch_route_owner_exists=1
  clone_share_semantics_preserved=1
  weak_behavior_defined=1
  fini_owner_defined=1
  backend_unsupported_surfaces_fail_fast=1
```

Only after this gate exists may a concrete Box family start Arc retirement.

## Do Not Do Yet

```text
do not replace Arc globally
do not add a cycle collector as an Arc-retirement prerequisite
do not make TypeAbiCatalog identity truth
do not change plugin ABI
do not rewrite Box trait surface
do not mix this side-lane with current exact-AOT optimization commits
```
