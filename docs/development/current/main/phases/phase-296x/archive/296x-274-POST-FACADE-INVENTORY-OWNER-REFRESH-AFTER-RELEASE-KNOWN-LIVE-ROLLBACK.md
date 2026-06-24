---
Status: Landed
Date: 2026-05-29
Scope: refresh owner after rejecting another facade keeper.
Blocker: POST-FACADE-INVENTORY-OWNER-REFRESH-AFTER-RELEASE-KNOWN-LIVE-ROLLBACK-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-273-FACADE-FIELD-OWNER-SELECTION-AFTER-RELEASE-KNOWN-LIVE-ROLLBACK.md
  - docs/development/current/main/phases/phase-296x/296x-270-POST-RELEASE-KNOWN-LIVE-RMW-ROLLBACK-OWNER-REFRESH.md
---

# 296x-274 Post Facade Inventory Owner Refresh After Release Known Live Rollback

## Purpose

Choose the next exact-slot owner after row273 rejected another facade keeper.

This row does not optimize. It reuses the row270 weighted owner table and
excludes the already-exercised facade surface plus the recent page-model
non-keeper, selecting the next unblocked family for an IR-shape inventory.

## Evidence

```text
output_contract=post-facade-inventory-owner-refresh-after-release-known-live-rollback-v0
input_contract=facade-field-owner-selection-after-release-known-live-rollback-v0
workload_id=representative-object-lifecycle-small-block-v0
source_exact_slot_get_set_pct=49.64
excluded_family_0=object_lifecycle_facade
excluded_reason_0=facade_positive_net_surface_already_exercised
excluded_family_1=page_model_hotpath
excluded_reason_1=recent_nonkeeper_requires_fresh_shape_before_retry
selected_family=alloc_result_capsule
selected_family_pct=8.71
selected_owner=alloc_result_capsule_ir_shape_inventory_after_release_known_live_rollback
selected_reason=top_unblocked_family_after_facade_small_surface_and_recent_page_model_nonkeeper
next_diagnostic=alloc_result_capsule_ir_shape_inventory_after_release_known_live_rollback
weighted_hot_candidate_score_required=1
ir_shape_diff_required_before_next_keeper=1
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
family_0_name=object_lifecycle_facade
family_0_pct=15.68
family_1_name=page_model_hotpath
family_1_pct=11.34
family_2_name=alloc_result_capsule
family_2_pct=8.71
family_3_name=page_queue_helpers
family_3_pct=7.74
family_4_name=release_result_capsule
family_4_pct=4.40
```

## Decision

```text
selected_owner=alloc_result_capsule_ir_shape_inventory_after_release_known_live_rollback
next_row=alloc_result_capsule_ir_shape_inventory_after_release_known_live_rollback
optimization_open=0
```

The next row should inventory alloc-result capsule IR shape before any
implementation. Do not reopen facade same-block fusion, immediate page-model
retry, page-queue retry, generic typed-field residence, provider activation,
replacement, hooks, or globals.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_post_facade_inventory_owner_refresh_after_release_known_live_rollback_guard.sh
```
