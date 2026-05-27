---
Status: Landed
Date: 2026-05-27
Scope: refresh source/MIR observation after the active field fast path rollback.
Blocker: HAKO-MIMALLOC-POST-ROLLBACK-SOURCE-MIR-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-104-HAKO-MIMALLOC-POST-ROLLBACK-ACTIVE-FIELD-FAST-PATH-MEASUREMENT.md
  - docs/development/current/main/design/hako-check-mir-observation-boundary-ssot.md
---

# 296x-105 Hako Mimalloc Post Rollback Source/MIR Refresh

## Purpose

Refresh source/MIR observation after the active field fast path rollback. Rank
active owners with measurement counters and MIR shape so the next keeper avoids
the row101 non-keeper pattern.

## Required Output

```text
output_contract=hako-mimalloc-hot-owner-rank-v0
input_contract=hako-mimalloc-post-rollback-active-field-fast-path-measurement-v0
method_count
active_method_count
inactive_surface_count
rejected_keeper=select_single_page_active_field_fast_path
selected_owner
selected_risk_kind
next_keeper
summary=ok
```

## Stop Line

Do not implement another keeper in this refresh row. Keep provider activation,
replacement, hooks, globals, and winner claims closed.

## Landed Evidence

```text
output_contract=hako-mimalloc-hot-owner-rank-v0
input_contract=hako-mimalloc-post-rollback-active-field-fast-path-measurement-v0
method_count=6
active_method_count=5
inactive_surface_count=1
rejected_keeper=select_single_page_active_field_fast_path
rejected_keeper_reason=measured_regression_row102
active_method_rank_0=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
active_method_rank_0_source_method=objectLifecycleSmallAlloc
active_method_rank_0_active_count=524288
active_method_rank_0_mir_call_count=24
active_method_rank_0_mir_field_access_count=16
active_method_rank_0_mir_array_access_count=0
active_method_rank_0_score=46137344
active_method_rank_0_risk_kind=method_call_surface
active_method_rank_1=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2
active_method_rank_1_source_method=objectLifecycleReleaseBlock
active_method_rank_1_active_count=524288
active_method_rank_1_mir_call_count=22
active_method_rank_1_mir_field_access_count=4
active_method_rank_1_mir_array_access_count=1
active_method_rank_1_score=38797312
active_method_rank_1_risk_kind=method_call_surface
active_method_rank_5=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseKnownPageIndex/1
active_method_rank_5_source_method=objectLifecycleReleaseKnownPageIndex
active_method_rank_5_active_count=0
selected_owner=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
selected_source_method=objectLifecycleSmallAlloc
selected_risk_kind=method_call_surface
selected_reason=top_active_owner_can_bypass_selectPage_wrapper_for_single_page_workload
selected_next_kind=keeper
next_keeper=small_alloc_direct_single_page_select_fast_path
next_keeper_kind=box_count
confidence=medium
next_row=HAKO-MIMALLOC-SMALL-ALLOC-DIRECT-SINGLE-PAGE-SELECT-FAST-PATH-KEEPER-296X-001
winner_claim=0
replacement_active=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_hot_owner_rank_guard.sh
```
