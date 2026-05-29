---
Status: Landed
Date: 2026-05-29
Scope: select whether existing helper fallback may route through explicit DirectSlot snapshots.
Blocker: DIRECT-SLOT-BACKEND-HELPER-FALLBACK-ROUTING-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-328-DIRECT-SLOT-BACKEND-MATERIALIZATION-SNAPSHOT-PILOT.md
  - crates/nyash_kernel/src/exports/typed_object_store.rs
---

# 296x-329 Direct Slot Backend Helper Fallback Routing Selection

## Purpose

Decide whether existing typed-object helpers may route through
`DirectSlotObjectV0` snapshot materialization.

The answer for hot helper fallback is no. Per-helper-call snapshot routing would
turn every fallback read/write into full view construction and would obscure the
DirectSlot primary-storage policy. Snapshot materialization remains valid as an
explicit boundary operation, not as a hidden per-call fallback path.

## Contract

```text
output_contract=direct-slot-backend-helper-fallback-routing-selection-v0
input_contract=direct-slot-backend-materialization-snapshot-pilot-v0
selected_owner=direct_slot_materialized_view_boundary_handle_policy
selected_reason=per_helper_snapshot_routing_would_reintroduce_work_explosion_and_hide_materialization_boundaries
existing_helper_route_to_direct_backend=0
existing_helper_route_to_snapshot_per_call=0
generic_helper_route_to_direct_backend=0
exact_slot_helper_route_to_direct_backend=0
snapshot_materialization_allowed=1
snapshot_materialization_trigger=explicit_boundary_only
materialized_view_handle_required_before_helper_routing=1
direct_cell_primary_storage=1
typed_slot_role=materialized_view_not_primary_storage
dual_truth_allowed=0
implicit_sync_on_every_direct_write=0
new_c_abi_helper_symbols=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
implementation_open=0
optimization_open=0
selected_next=direct_slot_materialized_view_handle_policy_selection
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

Do not route existing typed-object helpers through snapshots on each helper
call.

The next boundary is a materialized view handle policy:

```text
DirectSlotObjectV0 primary storage
  -> explicit boundary materialization
  -> separate TypedSlotObject view handle
  -> existing helpers may read the materialized view, not the DirectSlot object
```

This keeps materialization visible and prevents accidental dual truth. The
DirectSlot object remains the source of truth until an explicit boundary creates
a compatibility view.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_backend_helper_fallback_routing_selection_guard.sh
```
