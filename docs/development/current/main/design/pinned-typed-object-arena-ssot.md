---
Status: Active
Date: 2026-05-29
Scope: pinned typed-object arena contract required before DirectSlotLease / NativeDirect typed-object lowering.
Related:
  - docs/development/current/main/design/representation-direct-storage-substrate-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-306-TYPED-OBJECT-DIRECT-SLOT-LEASE-FEASIBILITY.md
---

# Pinned Typed Object Arena SSOT

## Purpose

Define the storage contract needed before typed-object `DirectSlotLease` can be
implemented.

Row306 proved that the current typed-object store cannot support a lease:

```text
object_storage_container=Vec<TypedSlotObject>
field_storage_container=Vec<TypedSlot>
object_generation_available=0
object_storage_pinned=0
field_address_stable=0
vec_reallocation_possible=1
direct_slot_lease_feasible_without_storage_change=0
required_runtime_storage_change=pinned_typed_object_arena
```

The pinned arena is therefore a storage substrate, not a lowering optimization.

## Decision

```text
output_contract=pinned-typed-object-arena-ssot-v0
input_contract=typed-object-direct-slot-lease-feasibility-v0
selected_design_owner=pinned_typed_object_arena
selected_reason=current_vec_refcell_store_cannot_support_direct_slot_lease
default_backend_unchanged=1
existing_helper_abi_unchanged=1
pinned_arena_backend_default=0
direct_lowering_open=0
native_direct_open=0
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Required Shape

`PinnedTypedObjectArena` must provide stable object and field identity for a
bounded compiler-selected region.

Required facts:

```text
object_storage_pinned_required=1
field_address_stable_required=1
object_generation_required=1
slot_layout_stable_required=1
handle_generation_validation_required=1
lease_region_required=1
lease_barrier_policy_required=1
```

The arena may use any internal representation that satisfies those facts. The
SSOT does not require exposing raw pointers from the existing runtime `Vec`.

## Handle And Generation

Handles remain opaque at the language/runtime boundary.

The pinned arena must support generation validation so stale handles cannot
silently access a recycled object slot:

```text
object_handle_is_identity_token=1
object_generation_available=1
stale_generation_access_fail_fast=1
silent_stale_handle_reuse_allowed=0
```

The exact bit layout is not fixed here. A later implementation row may choose
between handle encoding and a side table, but the validation result is part of
the arena contract.

## Slot Stability

DirectSlotLease requires stable slot access within the lease region:

```text
slot_index_constant_required=1
storage_class_known_required=1
field_address_or_offset_stable=1
slot_layout_mutation_inside_lease_allowed=0
```

If a field storage transition is required, the region must not produce a direct
lease. It must stay on the existing helper path.

## Lease Boundaries

The arena is allowed to support direct access only behind a proven lease.

Required invalidation barriers:

```text
unknown_call_barrier=1
unknown_escape_barrier=1
aliasing_write_barrier=1
storage_kind_change_barrier=1
object_recycle_barrier=1
thread_or_worker_boundary_barrier=1
```

The compiler may only lower a lease-backed region when every barrier has a known
policy.

## Backend Policy

The pinned arena is not the default runtime storage.

```text
default_backend=existing_safe_mutex_or_single_thread_exact_store
pinned_arena_backend_default=0
pinned_arena_requires_explicit_exact_lane=1
existing_helper_abi_unchanged=1
provider_activation=0
allocator_replacement=0
hook_installed=0
global_allocator=0
```

The first implementation row must be storage-only. It must not change LLVM
field lowering or emit NativeDirect accesses.

## Forbidden

```text
raw_runtime_vec_pointer_exposure_allowed=0
silent_fallback_after_lease_selection_allowed=0
direct_lowering_before_arena_guard_allowed=0
by_name_hako_alloc_special_case_allowed=0
```

The arena must not make the current `Vec<TypedSlotObject>` addressable by
contract. A later implementation can introduce a pinned arena side-by-side with
the existing stores.

## First Implementation Boundary

The next row may open only this boundary:

```text
first_implementation_boundary=pinned_typed_object_arena_storage_pilot
allowed_owner=typed_object_runtime_storage
allowed_scope=storage_allocation_generation_and_slot_stability
rejected_scope=llvm_lowering_direct_slot_lease_native_direct
```

Acceptance for that later row must prove:

```text
pinned_object_allocation_smoke=ok
generation_validation_smoke=ok
slot_stability_smoke=ok
default_backend_smoke=ok
existing_helper_abi_unchanged=1
direct_lowering_open=0
summary=ok
```
