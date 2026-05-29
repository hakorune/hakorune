---
Status: Landed
Date: 2026-05-29
Scope: define DirectArrayI64BufferV0 materialization and fallback sync policy before backend connection or lowering.
Blocker: DIRECT-ARRAY-I64-MATERIALIZATION-SYNC-SSOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-350-DIRECT-ARRAY-I64-BUFFER-V0-STORAGE-PILOT.md
  - crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs
---

# 296x-351 Direct Array I64 Materialization Sync SSOT

## Purpose

Define the source-of-truth and sync policy for `DirectArrayI64BufferV0` before
any backend connection or LLVM lowering opens.

Row350 proved a stable exact-i64 buffer layout. The next risk is dual storage:
`ArrayBox` public storage and `DirectArrayI64BufferV0` must not both become
independent truths. This row makes the DirectArray buffer primary only for a
selected NativeDirect region, with public `ArrayBox` as the materialized
fallback/debug view at explicit boundaries.

This row is docs/guard only.

## Contract

```text
output_contract=direct-array-i64-materialization-sync-ssot-v0
input_contract=direct-array-i64-buffer-v0-storage-pilot-v0
selected_owner=direct_array_i64_materialization_sync_policy
direct_array_primary_storage_policy=selected_native_direct_region
public_arraybox_primary_storage_policy=default_runtime_path
dual_truth_allowed=0
materialized_view_kind=public_arraybox_snapshot
materialization_direction=direct_array_to_public_arraybox
bootstrap_direction=public_arraybox_to_direct_array_deferred
helper_fallback_direction=public_arraybox_only_until_backend_connection
direct_array_helper_route_open=0
backend_connection_open=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
public_arraybox_semantics_unchanged=1
default_safe_rwlock_path_unchanged=1
existing_array_helper_abi_unchanged=1
arraybox_items_rwlock_exposure=0
array_slot_cache_vec_exposure=0
selected_plan_silent_fallback_allowed=0
unsupported_storage_policy=no_plan
append_grow_policy=no_plan_until_capacity_or_grow_policy
oob_policy=preserve_or_no_plan
materialization_boundary_public_observer=1
materialization_boundary_unknown_escape=1
materialization_boundary_generic_array_method=1
materialization_boundary_storage_kind_change=1
materialization_boundary_capacity_growth_required=1
materialization_boundary_debug_or_proof_observer=1
generation_validation_required_before_handle_route=1
backend_selection_required_before_helper_route=1
positive_net_helper_delta_required_before_lowering=1
provider_activation=0
host_replacement=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
selected_next=direct_array_i64_backend_connection_selection
summary=ok
```

## Source Of Truth

The direct buffer is not a cache beside `ArrayBox`.

```text
DirectArray selected region:
  DirectArrayI64BufferV0 is primary storage.
  Public ArrayBox is a materialized view only at explicit boundaries.

Default runtime path:
  Public ArrayBox remains primary storage.
  DirectArrayI64BufferV0 is not emitted and not observed.
```

This prevents a split-brain state where both representations appear live and
authoritative.

## Materialization Boundaries

The planner must stop or materialize before:

```text
public observer
unknown escape
generic ArrayBox method
storage kind change
capacity growth requirement
debug/proof observer
```

If the selected plan cannot prove a boundary policy, it must produce no plan
rather than silently falling back to helper calls.

## Deferred Work

This row does not implement:

```text
ArrayBox -> DirectArray bootstrap
DirectArray -> ArrayBox materialization
helper fallback routing
backend selection
LLVM lowering
```

Those require separate rows because each one changes a different ownership
boundary.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_array_i64_materialization_sync_ssot_guard.sh
```
