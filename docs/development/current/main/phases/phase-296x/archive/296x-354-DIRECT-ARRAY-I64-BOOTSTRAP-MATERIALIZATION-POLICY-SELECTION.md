---
Status: Landed
Date: 2026-05-29
Scope: select the first DirectArrayI64BufferV0 bootstrap/materialization bridge before helper routing or lowering.
Blocker: DIRECT-ARRAY-I64-BOOTSTRAP-MATERIALIZATION-POLICY-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-351-DIRECT-ARRAY-I64-MATERIALIZATION-SYNC-SSOT.md
  - docs/development/current/main/phases/phase-296x/296x-353-DIRECT-ARRAY-I64-BACKEND-CONNECTION-PILOT.md
  - crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs
  - crates/nyash_kernel/src/plugin/array_slot_backend.rs
---

# 296x-354 Direct Array I64 Bootstrap Materialization Policy Selection

## Purpose

Select the first bootstrap/materialization bridge for the `direct_array_i64_exact`
backend.

The bridge must preserve row351: `DirectArrayI64BufferV0` is primary storage only
inside a selected NativeDirect region, and public `ArrayBox` is a
fallback/materialization/debug view. This row does not implement the bridge,
does not route existing helpers to the DirectArray backend, and does not open
LLVM lowering.

## Contract

```text
output_contract=direct-array-i64-bootstrap-materialization-policy-selection-v0
input_contract=direct-array-i64-backend-connection-pilot-v0
selected_owner=direct_array_i64_to_public_arraybox_snapshot_materialization
selected_reason=existing_array_helpers_must_not_route_to_direct_backend_until_explicit_materialization_view_exists
selected_bridge=direct_array_i64_buffer_v0_to_public_arraybox_snapshot
sync_direction=direct_array_i64_to_public_arraybox_snapshot
bootstrap_direction=public_arraybox_to_direct_array_i64_deferred
direct_array_primary_storage=1
public_arraybox_role=fallback_materialization_debug_view
public_arraybox_primary_storage_in_direct_backend=0
dual_truth_allowed=0
implicit_sync_on_every_direct_write=0
per_write_public_arraybox_update=0
materialization_trigger=explicit_only
materialization_view_lifetime=snapshot
materialization_requires_direct_array_backend=1
materialization_requires_generation_validation=1
materialization_requires_i64_element_tag=1
materialization_requires_len_le_capacity=1
unsupported_element_tag_policy=fail_or_none_not_silent_fallback
generic_array_helper_route_to_direct_backend=0
i64_slot_helper_route_to_direct_backend=0
fallback_helper_reads_direct_array_before_bridge=0
helper_routing_implementation_open=0
materialization_bridge_implementation_open=0
bootstrap_bridge_implementation_open=0
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
selected_next=direct_array_i64_materialization_snapshot_pilot
summary=ok
```

## Decision

The first bridge is an explicit snapshot materialization:

```text
DirectArrayI64BufferV0
  -> public ArrayBox snapshot
```

This is not a permanent dual-storage policy. The snapshot is a compatibility
view for future helper fallback, debug, proof, and public observer boundaries.
Existing helpers remain closed for `direct_array_i64_exact` until a later row
implements and guards the snapshot bridge.

## Rejected

```text
rejected=route_existing_helpers_directly_to_direct_array
reason=no_materialization_view_exists_yet

rejected=update_public_arraybox_on_every_direct_write
reason=would_create_dual_truth_and_reintroduce_per_write_overhead

rejected=llvm_lowering_against_direct_array_i64_buffer
reason=materialization_and_helper_fallback_policy_not_implemented_yet
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_array_i64_bootstrap_materialization_policy_selection_guard.sh
```
