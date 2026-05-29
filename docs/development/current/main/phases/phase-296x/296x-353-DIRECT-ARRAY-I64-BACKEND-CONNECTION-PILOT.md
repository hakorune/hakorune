---
Status: Landed
Date: 2026-05-29
Scope: add the direct_array_i64_exact backend selection point without routing helpers or lowering.
Blocker: DIRECT-ARRAY-I64-BACKEND-CONNECTION-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-352-DIRECT-ARRAY-I64-BACKEND-CONNECTION-SELECTION.md
  - crates/nyash_kernel/src/plugin/array_slot_backend.rs
  - crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs
---

# 296x-353 Direct Array I64 Backend Connection Pilot

## Purpose

Add the `direct_array_i64_exact` backend selection point and prove it can
allocate `DirectArrayI64BufferV0` storage.

This row intentionally does not route existing Array helpers to the DirectArray
backend. Bootstrap/materialization sync is not implemented yet, so helper
fallback must fail fast for this backend instead of silently reading a second
truth.

## Evidence

```text
output_contract=direct-array-i64-backend-connection-pilot-v0
input_contract=direct-array-i64-backend-connection-selection-v0
implemented_backend=direct_array_i64_exact
implemented_owner=crates/nyash_kernel/src/plugin/array_slot_backend.rs
storage_owner=crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs
direct_array_i64_buffer_allocation_smoke=ok
direct_array_i64_buffer_store_load_smoke=ok
helper_route_to_direct_backend=fail_fast_closed
generic_array_helper_route_to_direct_backend=0
i64_slot_helper_route_to_direct_backend=0
materialization_bridge_implemented=0
bootstrap_bridge_implemented=0
existing_array_helper_abi_unchanged=1
default_backend_unchanged=1
single_thread_exact_backend_unchanged=1
new_c_abi_helper_symbols=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
provider_activation=0
host_replacement=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
selected_next=direct_array_i64_bootstrap_materialization_policy_selection
summary=ok
```

## Decision

`direct_array_i64_exact` is now a selectable backend name. It only proves that
the backend selector can name the future DirectArray primary-storage backend and
that storage construction works in isolation.

Existing helper calls must not use the DirectArray buffer yet:

```text
helper_route_to_direct_backend=fail_fast_closed
```

This keeps the DirectArray path from becoming a silent helper fallback before
the bootstrap/materialization policy exists.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_array_i64_backend_connection_pilot_guard.sh
```
