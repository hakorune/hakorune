---
Status: Landed
Date: 2026-05-29
Scope: roll back no-effect releaseLocalKnownLive single-use RMW implementation.
Blocker: PAGE-MODEL-RELEASE-KNOWN-LIVE-SINGLE-USE-RMW-ROLLBACK-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-268-PAGE-MODEL-RELEASE-KNOWN-LIVE-SINGLE-USE-RMW-MEASUREMENT.md
---

# 296x-269 Page Model Release Known Live Single-Use RMW Rollback

## Purpose

Roll back the row267 implementation after row268 measured it as no-effect.

This row preserves the observation/selection documents and removes only the
same-module target-list addition that enabled RMW fusion for
`HakoAllocPageModel.releaseLocalKnownLive/1`.

## Evidence

```text
output_contract=page-model-release-known-live-single-use-rmw-rollback-v0
input_contract=page-model-release-known-live-single-use-rmw-measurement-v0
workload_id=representative-object-lifecycle-small-block-v0
rollback_reason=keeper_effect_no_effect
removed_target=HakoAllocPageModel.releaseLocalKnownLive/1
preserved_target=HakoAllocPageModel.acquire_usize/1
preserved_facade_rmw_targets=1
removed_runtime_helper=0
removed_hako_source_change=0
semantic_proof_summary=ok
post_rollback_action=owner_refresh
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_owner_family=post_release_known_live_rmw_rollback_owner_refresh
selected_reason=no_effect_page_model_helper_tweak_was_removed_before_another_owner_attempt
next_row=post_release_known_live_rmw_rollback_owner_refresh
optimization_open=0
```

The next row should refresh hot ownership. It must not immediately try another
page-model helper tweak without new perf evidence.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_page_model_release_known_live_single_use_rmw_rollback_guard.sh
```
