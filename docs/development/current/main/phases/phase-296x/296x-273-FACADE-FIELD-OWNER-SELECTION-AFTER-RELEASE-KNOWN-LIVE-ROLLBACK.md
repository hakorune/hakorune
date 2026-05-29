---
Status: Landed
Date: 2026-05-29
Scope: select whether to reopen facade field optimization after rollback inventory.
Blocker: FACADE-FIELD-OWNER-SELECTION-AFTER-RELEASE-KNOWN-LIVE-ROLLBACK-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-272-FACADE-EXACT-SLOT-IR-SHAPE-DIFF-INVENTORY-AFTER-RELEASE-KNOWN-LIVE-ROLLBACK.md
---

# 296x-273 Facade Field Owner Selection After Release Known Live Rollback

## Purpose

Decide whether row272 facade inventory justifies another facade keeper.

This row keeps optimization closed. It rejects repeating the selected facade
same-block get/set fusion because that surface was already exercised by the
row231 keeper and the positive-net count is still only 4.

## Evidence

```text
output_contract=facade-field-owner-selection-after-release-known-live-rollback-v0
input_contract=object-lifecycle-facade-exact-slot-field-inventory-v0
workload_id=representative-object-lifecycle-small-block-v0
dominant_field_family=facade_receiver_state
facade_receiver_state_count=15
page_queue_bridge_count=9
alloc_result_capsule_count=4
same_block_get_set_count=3
same_receiver_repeated_get_count=1
positive_net_cache_candidate_count=4
previous_selected_facade_get_set_keeper_landed=1
previous_selected_facade_get_set_measurement_row=296x-231
selected_owner=post_facade_inventory_owner_refresh
selected_reason=selected_facade_fusion_already_landed_and_positive_net_surface_still_4
next_diagnostic=post_facade_inventory_owner_refresh_after_release_known_live_rollback
rejected_owner=repeat_selected_facade_same_block_get_set_fusion
rejected_reason=same_block_get_set_candidate_count_3_already_exercised_by_row231_keeper
rejected_owner_1=generic_typed_field_residence_retry
rejected_reason_1=no_new_family_specific_positive_net_plan
rejected_owner_2=facade_method_local_scalar_cache
rejected_reason_2=same_receiver_repeated_get_count_1_too_small
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_owner=post_facade_inventory_owner_refresh
next_row=post_facade_inventory_owner_refresh_after_release_known_live_rollback
optimization_open=0
```

Do not reopen selected facade same-block get/set fusion or generic typed-field
residence from this evidence. The next row should refresh exact-slot ownership
after excluding this already-exercised facade surface.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_facade_field_owner_selection_after_release_known_live_rollback_guard.sh
```
