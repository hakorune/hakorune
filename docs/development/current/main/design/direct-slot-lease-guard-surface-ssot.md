---
Status: Active
Date: 2026-05-29
Scope: first DirectSlotLease guard surface after pinned typed-object arena exact-slot fallback.
Related:
  - docs/development/current/main/design/representation-direct-storage-substrate-ssot.md
  - docs/development/current/main/design/pinned-typed-object-arena-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-312-PINNED-TYPED-OBJECT-ARENA-EXACT-SLOT-HELPER-PILOT.md
---

# Direct Slot Lease Guard Surface SSOT

## Purpose

Define the first `DirectSlotLease` guard surface.

This is the boundary between pinned typed-object storage and future
NativeDirect lowering. It does not emit LLVM direct loads/stores. It defines the
facts that must be true before such lowering can be selected later.

## Layer Split

```text
hako_alloc_policy_state_owner=unchanged
raw_memory_owner=capability_substrate_or_native_metal
representation_owner=compiler_direct_lowering
helper_path=fallback_materialization_debug
```

`DirectSlotLease` does not define allocator policy. It only describes a bounded
representation of already-proven typed-object state.

## First Guard Contract

```text
output_contract=direct-slot-lease-guard-surface-v0
input_contract=pinned-typed-object-arena-exact-slot-helper-pilot-v0
selected_owner=typed_object_direct_slot_lease_guard
selected_storage_backend=pinned_arena_exact
selected_storage_classes=i64|u64|handle
lease_token_runtime_smoke_open=1
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
helper_fallback_required=1
materialization_policy_required=1
barrier_policy_required=1
summary=ok
```

## Required Facts

A lease can be issued only when all facts are proven:

```text
backend_is_pinned_arena_exact=1
handle_generation_valid=1
receiver_type_known=1
slot_constant=1
storage_class_known=1
storage_class_supported=1
field_address_or_offset_stable=1
lease_region_known=1
materialization_policy_known=1
helper_fallback_available=1
```

Unsupported storage classes stay on exact-slot helpers.

## Barriers

The first guard surface treats every unknown mutation/escape boundary as a
lease barrier:

```text
unknown_call_barrier=1
unknown_escape_barrier=1
aliasing_write_barrier=1
storage_kind_change_barrier=1
object_recycle_barrier=1
thread_or_worker_boundary_barrier=1
```

Crossing a barrier ends the lease. Later lowering must either materialize
through the helper fallback or refuse the plan.

## Runtime Token Pilot

The next implementation row may add a runtime-only lease token API inside the
pinned arena module.

Allowed:

```text
lease_token_struct=1
lease_validate_i64_u64_handle=1
lease_read_write_smoke=1
existing_helper_abi_unchanged=1
default_backend_unchanged=1
```

Rejected:

```text
new_c_abi_helper_symbols=0
llvm_lowering_change=0
native_direct_lowering=0
raw_runtime_vec_pointer_exposure=0
by_name_hako_alloc_special_case=0
silent_fallback_after_lease_selection=0
```

## Fail-Fast

```text
lease selected and backend_is_pinned_arena_exact != 1
  -> fail-fast

lease selected and helper_fallback_available != 1
  -> fail-fast

lease selected and any barrier policy unknown
  -> fail-fast

lease selected and storage_class_supported != 1
  -> fail-fast
```

## Next Row

The next row may implement only:

```text
DIRECT-SLOT-LEASE-RUNTIME-TOKEN-PILOT-296X-001
```

That row proves a token can validate generation, storage class, and stable slot
access inside `pinned_arena_exact`. It must keep LLVM lowering closed.
