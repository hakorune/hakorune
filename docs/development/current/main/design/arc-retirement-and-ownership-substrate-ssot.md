---
Status: SSOT
Decision: accepted-for-tasking
Date: 2026-06-14
Scope: Arc retirement / ownership substrate boundary. This is a design
side-lane and does not change the active exact-AOT optimization lane.
Related:
  - docs/development/current/main/design/runtime-gc-policy-and-order-ssot.md
  - docs/development/current/main/phases/phase-29y/20-RC-INSERTION-SSOT.md
  - docs/development/current/main/phases/phase-29y/40-OPTIONAL-GC-LANE-ENTRY-SSOT.md
  - docs/development/current/main/design/box-callable-registry-ssot.md
  - docs/development/current/main/design/type-abi-catalog-planning-spine-ssot.md
  - docs/development/current/main/design/object-handle-box-identity-contract-ssot.md
  - docs/development/current/main/workstreams/arc-retirement-current.md
---

# Arc Retirement And Ownership Substrate (SSOT)

## Decision

Arc retirement is a future ownership-substrate lane, not a GC implementation
lane.

```text
gc_is_current_reclamation_owner=0
arc_is_current_reclamation_owner=1
rc_mir_emit_is_timing_owner=partial
arc_role_count=4
box_object_model_replacement_required=1
arc_hot_path_retirement_started=0
active_optimization_lane_changed=0
summary=accepted_for_tasking
```

Use this SSOT to task and gate future Arc retirement work. Do not start Arc
replacement from optimization evidence alone.

## Vocabulary

```text
RC insertion:
  MIR-level ownership event placement.
  It decides where release intent appears.

Ownership substrate:
  Runtime/object-table layer that owns refcount storage, atomic transitions,
  free-on-zero, root visibility, weak handles, and finalization.

Box object model:
  Identity, dispatch, clone/share semantics, type identity, plugin lifetime,
  and downcast/type tests.

GC / cycle collector:
  Optional policy/recipe layer above the ownership substrate.
  It is not the current physical reclamation owner.
```

## Current Reality

The current `GC` mode is diagnostic/reachability oriented. It does not reclaim
strong cycles and is not the physical free owner.

Evidence:

```text
src/runtime/gc_mode.rs:
  external modes auto|rc+cycle|off map to RcDiagnostic or Off
  rc+cycle is a compatibility spelling and does not reclaim strong cycles

src/runtime/gc_controller.rs:
  trial collection snapshots roots and traces reachability
  strong cycles are not reclaimed

docs/reference/runtime/gc.md:
  current rc+cycle is RC-backed diagnostics/reachability trials
```

Therefore:

```text
actual_current_non_cycle_reclamation=Rust_Arc_RC
cycle_collection_reclamation=not_implemented
gc_optional_policy_can_be_hako_recipe_later=1
```

## Arc Has Four Roles

`Arc<dyn NyashBox>` is not just a refcount. Retiring it requires replacing four
roles.

```text
1. inc/dec timing
   current: scattered/partial ReleaseStrong and runtime Arc clone/drop
   future: RC insertion pass emits ownership events from one owner

2. refcount storage
   current: Rust Arc allocation header
   future: object header or side table owned by ownership substrate

3. atomic inc/dec and free-on-zero
   current: Rust Arc implementation
   future: hako_atomic_* plus hako_mem_free / hako_alloc primitives

4. object model and dispatch
   current: dyn NyashBox, Any/TypeId, clone_box/share_box, Arc identity
   future: ObjectHandle / ObjectRef, BoxCallableRegistry, stable Box identity
```

Arc cannot retire until all four roles have replacement owners.

## Current Important Owners

### Core Box Trait

```text
src/boxes/box_trait.rs:
  SharedNyashBox = Arc<dyn NyashBox>
  clone_box / share_box / clone_arc define current sharing vocabulary
```

Future owner:

```text
ObjectHandle / ObjectRef contract
share semantics outside dyn trait-object cloning
```

### VM Value Carrier

```text
src/backend/vm_types.rs:
  VMValue::BoxRef(Arc<dyn NyashBox>)
  WeakBox = Weak<dyn NyashBox>
```

Future owner:

```text
VMValue carries ObjectHandle / compact ValueRef
object table owns boxed identity
```

### Host Handles

```text
src/runtime/host_handles.rs:
  global u64 -> Arc<dyn NyashBox> table
  borrowed read APIs already exist to avoid Arc clone cost
```

Future owner:

```text
handle table / object arena
u64 host ABI can remain stable while internals change
```

### Weak Handles

```text
src/runtime/weak_handles.rs:
  Weak<dyn NyashBox> registry
```

Future owner:

```text
generation-checked weak handles into object arena
```

### Plugin Boundary

```text
TypeBox ABI v2:
  external plugin execution ABI stays canonical

PluginBoxV2:
  Arc<PluginHandleInner> owns plugin instance id and finalizer state

PluginLoader route resolver:
  raw plugin method / birth / fini ids remain route truth

BoxCallableRegistry:
  callable route truth snapshot for builtin/plugin/user callables
```

Future owner:

```text
plugin instance table owns plugin handles
PluginBox carries stable plugin object handle
clone-vs-share semantics remain explicit
```

## RC Insertion Reality

RC insertion is not fully active or fully single-source today.

```text
src/mir/passes/rc_insertion.rs:
  default build is no-op/stat counting
  real pass is feature-gated behind rc-insertion-minimal

src/mir/instruction.rs:
  ReleaseStrong { values } exists

src/mir/compiler/mod.rs:
  compiler calls the pass after verification and before backend codegen

src/mir/builder/builder_build.rs:
  builder also directly emits ReleaseStrong on variable assignment overwrite
```

Consequences:

```text
rc_insertion_single_source=0
retain_strong_mir_instruction_exists=0
default_rc_insertion_active=0
release_strong_backend_parity_complete=0
```

Before Arc retirement implementation, close this gap or document it as a
deliberate compat seam with a retirement gate.

## Runtime Release Reality

```text
VM interpreter:
  ReleaseStrong removes BoxRef registers and SSA aliases with same Arc pointer

Python LLVM lowering:
  calls ny_release_strong(i64)

nyash_kernel:
  ny_release_strong aliases nyrt_handle_release_h

c-core shim:
  retain/release are no-op stubs

WASM:
  ReleaseStrong and KeepAlive are no-op
```

Arc retirement cannot claim backend-wide ownership until unsupported/no-op
backends are either implemented or fail-fast gated.

## Type ABI / BoxCallable Boundary

Type ABI must not become identity or dispatch truth.

```text
BoxCallableRegistry:
  callable-route truth snapshot

TypeAbiCatalog:
  read-only descriptor/query spine
  not hot execution truth
  not object identity truth

TypeBox ABI v2:
  external plugin ABI
```

Guard:

```text
box_callable_dispatch_truth_count>0
typeabi_identity_truth_count=0
typeabi_hot_lookup_count=0
```

## Non-Goals

```text
do not replace Arc in this tasking slice
do not change Box representation
do not change plugin ABI
do not make TypeAbiCatalog identity truth
do not add a cycle collector as a prerequisite for Arc retirement
do not reopen startup optimization from Arc retirement evidence
do not alter active exact-AOT optimization fronts
```

## Task Ladder

### ARC-RETIRE-000: SSOT and taskboard

```text
status=landed_when_this_doc_and_workstream_exist
implementation=none
active_optimization_lane_changed=0
```

### ARC-RETIRE-001: Arc role inventory

Inventory and classify:

```text
Arc<dyn NyashBox>
Box<dyn NyashBox>
Weak<dyn NyashBox>
Arc<PluginHandleInner>
Arc<Mutex<dyn NyashBox>>
Any / TypeId / parent_type_id
clone_box / share_box / clone_arc
```

Output:

```text
arc_role_inventory_count=717
arc_hot_path_count=nonzero
arc_compat_count=nonzero
arc_plugin_boundary_count=nonzero
arc_identity_truth_count=nonzero
```

Current inventory sample:

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

Classify:

```text
ReleaseStrong pass emission
builder direct ReleaseStrong emission
retain support
backend support
no-op backend surfaces
rc-insertion-minimal feature gates
```

Output:

```text
rc_insertion_single_source=0
default_rc_insertion_active=0
retain_strong_mir_instruction_exists=0
release_strong_backend_parity_complete=0
```

Current gap:

```text
ReleaseStrong_refs=62
RetainStrong_refs=0
builder_direct_release_strong=1
feature_gate=rc-insertion-minimal
```

### ARC-RETIRE-003: ObjectHandle / BoxIdentity contract

Contract owner:

```text
docs/development/current/main/design/object-handle-box-identity-contract-ssot.md
src/runtime/object_identity.rs
```

Defines:

```text
ObjectHandle
generation
WeakObjectHandle
RootVisibility
PluginInstanceIdentity
BuiltinIdentity
scope/fini ownership
```

Acceptance:

```text
object_handle_contract_defined=1
typeabi_identity_truth_count=0
arc_hot_path_retirement_started=0
```

### ARC-RETIRE-004: Host handle table as first seam

The safest first implementation seam is host handle internals, because the
public ABI can remain `u64`.

```text
external_host_abi_changed=0
object_handle_contract_used_by_host_handles=1
host_handle_identity_generation=legacy_unversioned
borrowed_access_preserved=1
identity_snapshot_available=1
host_handle_backing_arc_replaced=0
arc_hot_path_retirement_started=0
```

Implemented API surface:

```text
to_object_handle(raw_u64)
to_raw_handle(ObjectHandle)
identity(raw_u64)
descriptor(raw_u64)
with_object_handle(ObjectHandle, ...)
with_object_handle_ready(ObjectHandle, ...)
identity_snapshot()
host_handle_identity_report_fields()
```

This is a seam, not Arc retirement. Current host handles still store
`Arc<dyn NyashBox>`.

### ARC-RETIRE-005: Box object model replacement map

Map replacements for:

```text
dyn dispatch
clone_box
share_box
as_any/downcast
type_name
Send/Sync
finalization
plugin lifecycle
```

### ARC-RETIRE-006: Family-by-family retirement gates

A Box family may retire Arc only when:

```text
object_identity_owner_exists=1
refcount_storage_owner_exists=1
atomic_free_on_zero_exists=1
dispatch_route_owner_exists=1
clone_share_semantics_preserved=1
weak_behavior_defined=1
fini_owner_defined=1
backend_unsupported_surfaces_fail_fast=1
```

## Stop Line

```text
if a change only moves Arc behind a new wrapper, stop
if TypeAbiCatalog starts owning identity, stop
if plugin finalization owner becomes ambiguous, stop
if RC insertion gains another emission owner, stop
if a backend silently no-ops ownership events, stop
```

## Report Vocabulary

```text
arc_retirement_mode=tasking_only|inventory|implementation
arc_is_current_reclamation_owner=1
gc_is_current_reclamation_owner=0
gc_is_diagnostic_only=1
rc_mir_emit_is_timing_owner=partial|complete
rc_insertion_single_source=0|1
arc_role_count=4
box_object_model_replacement_required=1
typeabi_identity_truth_count=0
box_callable_dispatch_truth_count
object_handle_contract_defined=0|1
object_handle_contract_used_by_host_handles=0|1
host_handle_identity_generation=legacy_unversioned|versioned
borrowed_access_preserved=0|1
identity_snapshot_available=0|1
host_handle_backing_arc_replaced=0|1
plugin_lifecycle_owner_defined=0|1
arc_hot_path_retirement_started=0|1
summary=ok|fail
```
