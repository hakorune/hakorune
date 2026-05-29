---
Status: Landed
Date: 2026-05-29
Scope: select how DirectArrayI64BufferV0 connects to the Array slot backend without opening helper routing or lowering.
Blocker: DIRECT-ARRAY-I64-BACKEND-CONNECTION-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-351-DIRECT-ARRAY-I64-MATERIALIZATION-SYNC-SSOT.md
  - crates/nyash_kernel/src/plugin/array_slot_backend.rs
  - crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs
---

# 296x-352 Direct Array I64 Backend Connection Selection

## Purpose

Select the backend connection point for `DirectArrayI64BufferV0`.

The connection must stay runtime/storage-only. It must not make public
`ArrayBox` use the direct buffer by default, it must not route existing helpers
to the direct buffer, and it must not open LLVM lowering.

## Contract

```text
output_contract=direct-array-i64-backend-connection-selection-v0
input_contract=direct-array-i64-materialization-sync-ssot-v0
selected_owner=array_slot_backend_direct_array_i64_connection
selected_backend_name=direct_array_i64_exact
selected_reason=connect_stable_direct_array_i64_storage_before_lowering_retry
new_backend_allowed=1
default_backend_unchanged=1
single_thread_exact_backend_unchanged=1
direct_array_buffer_storage_owner=crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs
array_slot_backend_selector_owner=crates/nyash_kernel/src/plugin/array_slot_backend.rs
direct_array_primary_storage=DirectArrayI64BufferV0
public_arraybox_role=fallback_materialization_debug_view
array_slot_cache_role=diagnostic_helper_floor_only
direct_array_emission_default=0
generic_array_helper_route_to_direct_backend=0
i64_slot_helper_route_to_direct_backend=0
materialization_bridge_required_before_helper_route=1
bootstrap_bridge_required_before_helper_route=1
generation_validation_required_before_handle_route=1
new_c_abi_helper_symbols=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
implementation_open=0
optimization_open=0
provider_activation=0
host_replacement=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
selected_next=direct_array_i64_backend_connection_pilot
summary=ok
```

## Decision

Add a distinct backend selection point named `direct_array_i64_exact`.

This backend is not the same as `single_thread_exact`:

- `single_thread_exact` is a diagnostic helper floor with a small handle-entry
  cache;
- `direct_array_i64_exact` is the future primary-storage backend for
  `DirectArrayI64BufferV0`.

The pilot may add backend selection and isolated storage construction smoke
only. Existing Array helpers must keep using `safe_rwlock` or
`single_thread_exact` until bootstrap/materialization bridges are implemented.

## Closed Work

```text
generic_array_helper_route_to_direct_backend=0
i64_slot_helper_route_to_direct_backend=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
```

The next row must not silently make existing helper calls hit the DirectArray
buffer. That would create dual truth before materialization is implemented.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_array_i64_backend_connection_selection_guard.sh
```
