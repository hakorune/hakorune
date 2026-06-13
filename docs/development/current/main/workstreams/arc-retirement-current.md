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
arc_retirement_mode=first_real_family_cutover
arc_family_retirement_started=1
arc_hot_path_retirement_started=0
active_optimization_lane_changed=0
```

Arc retirement is allowed as docs/inventory planning, contract-only runtime
types, host-handle identity seam work, and Box object model replacement-map
work now. ARC-RETIRE-006..018 additionally defines the first family gate, cuts
over host-handle text payloads from Arc-backed payloads to direct `String`
payloads, and adopts that direct text carrier from the owned-text producer
boundary. Global Arc replacement has not started.

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

Status:

```text
landed_by=
  docs/development/current/main/design/arc-retirement-and-ownership-substrate-ssot.md
```

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
arc_role_inventory_count=717
arc_identity_truth_count=nonzero
arc_plugin_boundary_count=nonzero
arc_hot_path_count=nonzero
arc_compat_count=nonzero
```

Current count sample:

```text
Arc<dyn NyashBox=292
Weak<dyn NyashBox=10
SharedNyashBox=12
Arc<PluginHandleInner=3
Arc<Mutex<dyn NyashBox=4
clone_arc=1
clone_box=283
share_box=112
```

### ARC-RETIRE-002: RC MIR coverage inventory

Status:

```text
landed_by=
  docs/development/current/main/design/arc-retirement-and-ownership-substrate-ssot.md
```

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
rc_insertion_single_source=0
default_rc_insertion_active=0
retain_strong_mir_instruction_exists=0
release_strong_backend_parity_complete=0
```

### ARC-RETIRE-003: ObjectHandle / BoxIdentity contract

Status:

```text
landed_by=
  docs/development/current/main/design/object-handle-box-identity-contract-ssot.md
  src/runtime/object_identity.rs
```

Contract-only design before Arc replacement.

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
arc_hot_path_retirement_started=0
```

### ARC-RETIRE-004: Host handle table seam

Status:

```text
landed_by=
  src/runtime/host_handles.rs
```

First implementation seam after ARC-RETIRE-003. External host ABI remains
`u64`; current backing storage remains `Arc<dyn NyashBox>`.

```text
goal:
  keep external u64 handle ABI stable
  replace/classify backing Arc table responsibilities
  preserve borrowed access APIs
```

Slices:

```text
ARC-RETIRE-004A:
  to_object_handle(raw_u64) / to_raw_handle(ObjectHandle)
  external_host_abi_changed=0

ARC-RETIRE-004B:
  identity(raw_u64) -> BoxIdentity::legacy(ObjectHandle)
  host_handle_identity_generation=legacy_unversioned

ARC-RETIRE-004C:
  with_object_handle(ObjectHandle, ...)
  with_object_handle_ready(ObjectHandle, ...)
  borrowed_access_preserved=1

ARC-RETIRE-004D:
  descriptor(raw_u64) -> ObjectIdentityDescriptor
  identity_snapshot() -> Vec<ObjectIdentityDescriptor>
  identity_snapshot_available=1

ARC-RETIRE-004E:
  host_handle_identity_report_fields()
  object_handle_contract_used_by_host_handles=1
  host_handle_backing_arc_replaced=0
```

Acceptance:

```text
external_host_abi_changed=0
object_handle_contract_used_by_host_handles=1
host_handle_identity_generation=legacy_unversioned
borrowed_access_preserved=1
identity_snapshot_available=1
host_handle_backing_arc_replaced=0
arc_hot_path_retirement_started=0
```

Non-goals:

```text
no plugin ABI change
no Box trait rewrite
no global Arc removal claim
```

### ARC-RETIRE-005: Box object model replacement map

Status:

```text
landed_by=
  docs/development/current/main/design/box-object-model-replacement-map-ssot.md
  src/runtime/box_object_model.rs
  src/backend/vm_types.rs
```

Docs + contract inventory before any family Arc retirement.

```text
slices:
  ARC-RETIRE-005A:
    clone/share semantics inventory

  ARC-RETIRE-005B:
    dyn dispatch / as_any / TypeId surface inventory

  ARC-RETIRE-005C:
    plugin lifecycle owner map

  ARC-RETIRE-005D:
    VMValue::BoxRef carrier migration plan
```

Acceptance:

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

Non-goals:

```text
no VMValue::BoxRef layout change
no plugin ABI change
no Box trait rewrite
no family Arc retirement claim
```

### ARC-RETIRE-006: Family retirement gate

Status:

```text
landed_by=
  docs/development/current/main/design/arc-retirement-family-gate-and-first-family-ssot.md
  src/runtime/arc_retirement.rs
```

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

Acceptance:

```text
arc_retirement_family_gate_defined=1
arc_retirement_family_gate_satisfied=1
object_identity_owner_exists=1
refcount_storage_owner_exists=1
atomic_free_on_zero_exists=1
dispatch_route_owner_exists=1
clone_share_semantics_preserved=1
weak_behavior_defined=1
fini_owner_defined=1
backend_unsupported_surfaces_fail_fast=1
```

### ARC-RETIRE-007: First candidate family selection

Status:

```text
landed_by=
  docs/development/current/main/design/arc-retirement-family-gate-and-first-family-ssot.md
  src/runtime/arc_retirement.rs
```

Selected first family:

```text
first_arc_retirement_candidate=host_handle_text_payload
first_arc_retirement_scope=host_handle_carrier
```

This selection is intentionally host-handle text-only. It does not claim global
host-handle carrier replacement.

### ARC-RETIRE-008: Refcount storage owner prototype

Status:

```text
landed_by=
  docs/development/current/main/design/arc-retirement-family-gate-and-first-family-ssot.md
  src/runtime/arc_retirement.rs
```

Current first family:

```text
refcount_storage_owner_defined=1
refcount_storage_strategy=immediate_scalar_no_refcount
```

The selected family is owned directly by the handle slot. Future object
families still require object header or side-table storage before global Box
trait carrier retirement.

### ARC-RETIRE-009: Atomic retain/release/free-on-zero contract

Status:

```text
landed_by=
  docs/development/current/main/design/arc-retirement-family-gate-and-first-family-ssot.md
  src/runtime/arc_retirement.rs
```

Contract:

```text
atomic_retain_release_contract_defined=1
retain_symbol=hako_atomic_slot_fetch_add_i64
release_symbol=hako_atomic_slot_fetch_add_i64
release_uses_fetch_add_minus_one=1
free_symbol=hako_mem_free
```

### ARC-RETIRE-010: First-family Arc-retirement scaffold

Status:

```text
landed_by=
  docs/development/current/main/design/arc-retirement-family-gate-and-first-family-ssot.md
  src/runtime/arc_retirement.rs
```

Acceptance:

```text
first_family_arc_retirement_scaffold=1
first_family_carrier=stable_text_payload
first_family_host_handle_text_arc_free=1
first_family_box_trait_arc_replaced=0
global_arc_replaced=0
typeabi_identity_truth_count=0
```

This closes the first real family carrier cutover for host-handle text
payloads. It does not change `VMValue::BoxRef`, plugin carriers, or global
`dyn NyashBox`.

### ARC-RETIRE-011: Object refcount storage design

Status:

```text
landed_by=
  docs/development/current/main/design/arc-retirement-family-gate-and-first-family-ssot.md
  src/runtime/arc_retirement.rs
```

Acceptance:

```text
refcount_storage_owner_defined=1
refcount_storage_strategy=immediate_scalar_no_refcount
```

### ARC-RETIRE-012: Retain/release MIR contract

Status:

```text
landed_by=
  docs/development/current/main/design/arc-retirement-family-gate-and-first-family-ssot.md
  src/runtime/arc_retirement.rs
```

Acceptance:

```text
atomic_retain_release_contract_defined=1
retain_symbol=hako_atomic_slot_fetch_add_i64
release_symbol=hako_atomic_slot_fetch_add_i64
release_uses_fetch_add_minus_one=1
free_symbol=hako_mem_free
```

### ARC-RETIRE-013: ObjectHandle-backed object table prototype

Status:

```text
landed_by=
  src/runtime/host_handles.rs
```

Acceptance:

```text
object_handle_contract_used_by_host_handles=1
host_handle_text_payload_arc_replaced=1
```

### ARC-RETIRE-014: WeakObjectHandle behavior

Status:

```text
landed_by=
  docs/development/current/main/design/arc-retirement-family-gate-and-first-family-ssot.md
  src/runtime/object_identity.rs
```

Acceptance:

```text
weak_behavior_defined=1
weak_object_handle_generation_check_required=1
```

### ARC-RETIRE-015: First real object family selection

Status:

```text
landed_by=
  docs/development/current/main/design/arc-retirement-family-gate-and-first-family-ssot.md
  src/runtime/arc_retirement.rs
```

Acceptance:

```text
first_arc_retirement_candidate=host_handle_text_payload
first_arc_retirement_scope=host_handle_carrier
```

### ARC-RETIRE-016: First real family Arc carrier cutover

Status:

```text
landed_by=
  src/runtime/host_handles.rs
  src/runtime/arc_retirement.rs
```

Acceptance:

```text
first_family_arc_retirement_scaffold=1
first_family_carrier=stable_text_payload
first_family_host_handle_text_arc_free=1
first_family_box_trait_arc_replaced=0
global_arc_replaced=0
```

### ARC-RETIRE-017: Text payload producer adoption audit

Status:

```text
landed_by=
  docs/development/current/main/design/arc-retirement-family-gate-and-first-family-ssot.md
```

Acceptance:

```text
first_family_text_producer_audit=1
producer_audit_target=string_handle_from_owned_with_site
producer_audit_target=shared_empty_string_handle
text_producer_need_stable_object_kept_object=1
```

Audit result:

```text
string_handle_from_owned_with_site:
  safe producer target; owns String and does not require object identity

shared_empty_string_handle:
  safe producer target; singleton text payload has no fini owner

PublishReason::NeedStableObject:
  not a cutover target; remains object-backed
```

### ARC-RETIRE-018: Text payload producer cutover

Status:

```text
landed_by=
  crates/nyash_kernel/src/plugin/value_codec/string_materialize.rs
  crates/nyash_kernel/src/exports/string_helpers/materialize.rs
  crates/nyash_kernel/src/exports/string_helpers/concat.rs
  src/runtime/arc_retirement.rs
```

Acceptance:

```text
first_family_text_producer_cutover=1
text_producer_string_handle_from_owned_arc_free=1
text_producer_shared_empty_arc_free=1
text_producer_need_stable_object_kept_object=1
text_producer_compat_get_materializes=1
first_family_box_trait_arc_replaced=0
global_arc_replaced=0
```

Cutover:

```text
string_handle_from_owned_with_site:
  freezes owned bytes for existing accounting
  publishes direct text payload with host_handles::to_handle_text

shared_empty_string_handle:
  uses host_handles::to_handle_text

concat pair cache:
  no longer calls host_handles::get on text payload handles just to seed an Arc cache
```

## Do Not Do Yet

```text
do not replace Arc globally
do not add a cycle collector as an Arc-retirement prerequisite
do not make TypeAbiCatalog identity truth
do not change plugin ABI
do not rewrite Box trait surface
do not mix this side-lane with current exact-AOT optimization commits
```
