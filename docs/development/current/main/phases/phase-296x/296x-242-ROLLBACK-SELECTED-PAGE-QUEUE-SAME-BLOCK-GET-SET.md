---
Status: Landed
Date: 2026-05-29
Scope: roll back the selected page queue same-block get/set fusion non-keeper.
Blocker: ROLLBACK-SELECTED-PAGE-QUEUE-SAME-BLOCK-GET-SET-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-241-SELECTED-PAGE-QUEUE-SAME-BLOCK-GET-SET-MEASUREMENT.md
---

# 296x-242 Rollback Selected Page Queue Same-Block Get/Set

## Purpose

Roll back the row240 page queue target extension after row241 measured it as a
performance non-keeper.

This row preserves the earlier facade/page-model typed-field RMW fusion and
removes only the page queue target expansion.

## Evidence

```text
output_contract=rollback-selected-page-queue-same-block-get-set-v0
input_contract=selected-page-queue-same-block-get-set-measurement-v0
workload_id=representative-object-lifecycle-small-block-v0
rollback_reason=selected_page_queue_get_set_keeper_no_effect
removed_target_family=page_queue_helpers
facade_fusion_preserved=1
page_model_acquire_fusion_preserved=1
page_queue_fusion_target_removed=1
rmw_plan_capacity=16
semantic_proof_summary=ok
single_thread_backend_smoke=ok
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_next=post_page_queue_rollback_owner_refresh
next_row=post_page_queue_rollback_owner_refresh
```

The page queue fusion should not remain in the active implementation because
its sample-count 3 measurement was a non-keeper. The next row should refresh
the hot owner from the restored implementation.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_rollback_selected_page_queue_same_block_get_set_guard.sh
```
