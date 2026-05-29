---
Status: Landed
Date: 2026-05-30
Scope: select how explicit DirectArray materialization publishes a separate helper-compatible public ArrayBox view handle.
Blocker: DIRECT-ARRAY-I64-MATERIALIZED-VIEW-HANDLE-POLICY-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-355-DIRECT-ARRAY-I64-MATERIALIZATION-SNAPSHOT-PILOT.md
  - crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs
---

# 296x-356 Direct Array I64 Materialized View Handle Policy Selection

## Purpose

Select the handle policy for explicit DirectArray materialization.

The materialized view must be helper-compatible without making public
`ArrayBox` a second source of truth for the DirectArray buffer. Therefore the
view gets a separate public ArrayBox host handle. Existing Array helpers may
operate on that view handle only after an explicit materialization boundary
creates it.

## Contract

```text
output_contract=direct-array-i64-materialized-view-handle-policy-selection-v0
input_contract=direct-array-i64-materialization-snapshot-pilot-v0
selected_owner=direct_array_i64_materialized_public_arraybox_handle
selected_reason=array_helpers_require_public_arraybox_handle_not_per_call_snapshot_routing
direct_array_handle_kind=backend_internal_direct_array_buffer
direct_array_handle_public=0
materialized_view_handle_kind=public_arraybox_host_handle
materialized_view_handle_sign=positive_host_handle
materialized_view_storage=host_handle_arc_arraybox_snapshot
materialized_view_source=explicit_direct_array_i64_snapshot
materialized_view_lifetime=host_handle_lifetime
existing_helper_route_to_direct_backend=0
existing_helper_route_to_materialized_view_handle=1
existing_helper_route_to_snapshot_per_call=0
direct_array_primary_storage=1
public_arraybox_role=materialized_view_not_primary_storage
view_writeback_to_direct_array=0
dual_truth_allowed=0
implicit_sync_on_every_direct_write=0
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
selected_next=direct_array_i64_materialized_view_handle_pilot
summary=ok
```

## Decision

Use two object identities:

```text
DirectArray buffer:
  backend-internal DirectArrayI64BufferV0 object
  not a public ArrayBox handle

Materialized view:
  separate public ArrayBox host handle
  created from an explicit DirectArray snapshot
```

Existing Array helpers remain closed for the DirectArray backend object. They
may operate on the materialized public ArrayBox handle after explicit
materialization creates that view.

## Rejected

```text
rejected=helper_reads_direct_array_buffer
reason=would_hide_materialization_and_make_fallback_path_look_like_primary_storage

rejected=materialized_view_writes_back_to_direct_array
reason=would_reintroduce_dual_truth_before_a_writeback_policy_exists

rejected=per_call_snapshot_view
reason=work_explosion_and_hidden_materialization
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_array_i64_materialized_view_handle_policy_selection_guard.sh
```
