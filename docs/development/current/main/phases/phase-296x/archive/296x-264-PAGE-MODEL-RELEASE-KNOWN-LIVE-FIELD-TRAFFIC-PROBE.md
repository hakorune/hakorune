---
Status: Landed
Date: 2026-05-29
Scope: probe field/copy traffic inside HakoAllocPageModel.releaseLocalKnownLive/1.
Blocker: PAGE-MODEL-RELEASE-KNOWN-LIVE-FIELD-TRAFFIC-PROBE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-263-PAGE-MODEL-HOTPATH-SHAPE-OWNER-SELECTION-AFTER-RESULT-CAPSULE-RESET.md
---

# 296x-264 Page Model Release Known Live Field Traffic Probe

## Purpose

Inspect `HakoAllocPageModel.releaseLocalKnownLive/1` field and copy traffic
before any page-model implementation.

This row keeps optimization closed. It exists because `acquire_usize/1` is still
the top page-model method, but its current copy-materialization owner overlaps
the row252 no-material receiver-forwarding path.

## Evidence

```text
output_contract=page-model-release-known-live-field-traffic-probe-v0
input_contract=page-model-hotpath-shape-owner-selection-v0
workload_id=representative-object-lifecycle-small-block-v0
target_method=HakoAllocPageModel.releaseLocalKnownLive/1
target_method_pct=4.14
block_count=7
field_get_count=7
field_set_count=5
field_op_count=12
copy_count=13
call_count=2
branch_count=2
array_set_call_count=2
array_bridge_field_get_count=2
scalar_counter_field_op_count=10
same_block_get_set_count=4
rmw_candidate_count=4
rmw_single_use_candidate_count=2
rmw_multi_use_candidate_count=2
receiver_copy_count=3
recent_acquire_usize_copy_retry_blocked=1
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
field_0_name=block_used
field_0_get_count=1
field_0_set_count=0
field_1_name=local_free
field_1_get_count=1
field_1_set_count=0
field_2_name=local_free_count
field_2_get_count=1
field_2_set_count=1
field_3_name=local_free_top
field_3_get_count=1
field_3_set_count=1
field_4_name=retire_count
field_4_get_count=1
field_4_set_count=1
field_5_name=retired
field_5_get_count=1
field_5_set_count=1
field_6_name=used
field_6_get_count=1
field_6_set_count=1
same_block_get_set_field_0=local_free_top
same_block_get_set_field_1=local_free_count
same_block_get_set_field_2=used
same_block_get_set_field_3=retire_count
selected_next=page_model_release_known_live_owner_selection
summary=ok
```

## Decision

```text
selected_owner_family=page_model_release_known_live_owner_selection
selected_reason=release_known_live_has_four_same_block_rmw_candidates_and_two_array_bridge_field_gets_but_no_implementation_owner_selected_yet
next_row=page_model_release_known_live_owner_selection
optimization_open=0
```

The next row must select one owner from this probe. It should prefer a narrow
owner with positive helper-call delta and must keep the blocked
`acquire_usize/1` receiver-forwarding retry closed.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_page_model_release_known_live_field_traffic_probe_guard.sh
```
