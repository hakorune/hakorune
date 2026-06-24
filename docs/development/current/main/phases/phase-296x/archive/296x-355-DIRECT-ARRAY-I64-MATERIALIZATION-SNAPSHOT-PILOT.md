---
Status: Landed
Date: 2026-05-30
Scope: implement explicit DirectArrayI64BufferV0 to public ArrayBox snapshot materialization without helper routing or lowering.
Blocker: DIRECT-ARRAY-I64-MATERIALIZATION-SNAPSHOT-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-354-DIRECT-ARRAY-I64-BOOTSTRAP-MATERIALIZATION-POLICY-SELECTION.md
  - crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs
---

# 296x-355 Direct Array I64 Materialization Snapshot Pilot

## Purpose

Implement explicit snapshot materialization from `DirectArrayI64BufferV0` to a
public `ArrayBox` view.

This row keeps DirectArray primary storage separate from public ArrayBox
semantics. The snapshot is created only on explicit request; helper fallback,
backend bootstrap, handle publishing, and LLVM lowering remain closed.

## Evidence

```text
output_contract=direct-array-i64-materialization-snapshot-pilot-v0
input_contract=direct-array-i64-bootstrap-materialization-policy-selection-v0
implemented_owner=crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs
implemented_bridge=direct_array_i64_buffer_v0_to_public_arraybox_snapshot
sync_direction=direct_array_i64_to_public_arraybox_snapshot
materialization_trigger=explicit_only
materialization_view_lifetime=snapshot
direct_array_primary_storage=1
public_arraybox_role=fallback_materialization_debug_view
public_arraybox_primary_storage_in_direct_backend=0
dual_truth_allowed=0
implicit_sync_on_every_direct_write=0
per_write_public_arraybox_update=0
materialization_requires_generation_validation=1
materialization_requires_i64_element_tag=1
materialization_requires_len_le_capacity=1
unsupported_element_tag_policy=none_not_silent_fallback
public_arraybox_snapshot_smoke=ok
snapshot_len_preserved=1
snapshot_i64_values_preserved=1
snapshot_oob_semantics_preserved=1
generic_array_helper_route_to_direct_backend=0
i64_slot_helper_route_to_direct_backend=0
helper_routing_implementation_open=0
bootstrap_bridge_implementation_open=0
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
selected_next=direct_array_i64_materialized_view_handle_policy_selection
summary=ok
```

## Decision

`DirectArrayI64BufferV0` can now produce an explicit public `ArrayBox` snapshot:

```text
DirectArrayI64BufferV0
  -> ArrayBox::new()
  -> slot_store_i64_raw for each live element
```

The snapshot is a compatibility/debug/proof view. It is not kept synchronized
with later direct writes, and existing helpers do not route through it yet.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_array_i64_materialization_snapshot_pilot_guard.sh
```
