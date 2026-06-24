---
Status: Landed
Date: 2026-05-29
Scope: freeze releaseLocalKnownLive single-use RMW guard surface before implementation.
Blocker: PAGE-MODEL-RELEASE-KNOWN-LIVE-SINGLE-USE-RMW-GUARD-SURFACE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-265-PAGE-MODEL-RELEASE-KNOWN-LIVE-OWNER-SELECTION.md
---

# 296x-266 Page Model Release Known Live Single-Use RMW Guard Surface

## Purpose

Freeze the exact `releaseLocalKnownLive/1` single-use RMW surface before any
implementation.

This row does not change lowering. It limits the future implementation to two
positive-net `usize` counter fields and explicitly rejects multi-use RMW fields
and Array bridge field traffic.

## Evidence

```text
output_contract=page-model-release-known-live-single-use-rmw-guard-surface-v0
input_contract=page-model-release-known-live-owner-selection-v0
workload_id=representative-object-lifecycle-small-block-v0
target_method=HakoAllocPageModel.releaseLocalKnownLive/1
selected_owner=page_model_release_known_live_single_use_rmw_guard_surface
implementation_owner=c_abi_same_module_typed_field_rmw_fusion
existing_helper_symbol=nyash.object.exact_slot_rmw_add_u64_hiii
new_runtime_helper_required=0
candidate_count=2
candidate_0_field=local_free_count
candidate_0_slot=11
candidate_0_storage=usize_u64
candidate_0_delta=1
candidate_0_block=129
candidate_0_old_get_single_use=1
candidate_1_field=retire_count
candidate_1_slot=17
candidate_1_storage=usize_u64
candidate_1_delta=1
candidate_1_block=133
candidate_1_old_get_single_use=1
planned_erased_helper_calls=4
planned_added_helper_calls=2
planned_net_helper_call_delta=2
multi_use_rmw_rejected=1
multi_use_rmw_field_0=local_free_top
multi_use_rmw_field_1=used
array_bridge_rejected=1
array_bridge_field_0=block_used
array_bridge_field_1=local_free
exact_method_only=1
same_module_only=1
source_rewrite=0
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_owner_family=page_model_release_known_live_single_use_rmw_implementation
selected_reason=two_single_use_usize_counter_pairs_can_reuse_existing_u64_rmw_helper_with_net_helper_delta_two
next_row=page_model_release_known_live_single_use_rmw_implementation
optimization_open=0
```

The implementation row may reuse the existing
`nyash.object.exact_slot_rmw_add_u64_hiii` helper for the two selected `usize`
counter fields only. It must not fuse `local_free_top` or `used`, because those
source values are used by other expressions in the same method and do not
guarantee positive helper-call delta.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_page_model_release_known_live_single_use_rmw_guard_surface_guard.sh
```
