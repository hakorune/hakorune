---
Status: Landed
Date: 2026-05-30
Scope: implement explicit DirectArray materialization into a separate public ArrayBox host handle.
Blocker: DIRECT-ARRAY-I64-MATERIALIZED-VIEW-HANDLE-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-356-DIRECT-ARRAY-I64-MATERIALIZED-VIEW-HANDLE-POLICY-SELECTION.md
  - crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs
---

# 296x-357 Direct Array I64 Materialized View Handle Pilot

## Purpose

Implement explicit DirectArray materialization into a separate helper-compatible
public ArrayBox host handle.

The DirectArray backend object remains non-public. Existing helpers do not route
to the DirectArray buffer. A caller must explicitly materialize a snapshot to
obtain a public ArrayBox host handle.

## Evidence

```text
output_contract=direct-array-i64-materialized-view-handle-pilot-v0
input_contract=direct-array-i64-materialized-view-handle-policy-selection-v0
implemented_owner=crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs
implemented_api=materialize_public_arraybox_snapshot_handle
materialized_view_storage=host_handle_arc_arraybox_snapshot
direct_array_handle_public=0
materialized_view_handle_sign=positive_host_handle
materialized_view_source=explicit_direct_array_i64_snapshot
materialized_view_lifetime=host_handle_lifetime
direct_array_helper_route=closed
materialized_view_helper_route=public_arraybox_existing_helpers
view_writeback_to_direct_array=0
direct_array_primary_storage=1
public_arraybox_role=materialized_view_not_primary_storage
dual_truth_allowed=0
implicit_sync_on_every_direct_write=0
generic_array_helper_route_to_direct_backend=0
i64_slot_helper_route_to_direct_backend=0
new_c_abi_helper_symbols=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
public_arraybox_host_handle_smoke=ok
snapshot_len_preserved=1
snapshot_i64_values_preserved=1
provider_activation=0
host_replacement=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
selected_next=direct_array_i64_helper_fallback_closeout_and_lowering_readiness_selection
summary=ok
```

## Decision

The materialized view handle is a normal positive host handle that points to a
separate public `ArrayBox` snapshot.

This makes helper compatibility explicit:

```text
DirectArrayI64BufferV0
  -> explicit snapshot
  -> public ArrayBox host handle
  -> existing Array helpers
```

There is still no helper route from existing Array helpers to the DirectArray
buffer itself.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_array_i64_materialized_view_handle_pilot_guard.sh
```
