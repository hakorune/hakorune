---
Status: Landed
Date: 2026-05-27
Scope: refresh source/MIR observation after the small-alloc direct select keeper measurement.
Blocker: HAKO-MIMALLOC-POST-SMALL-ALLOC-DIRECT-SELECT-SOURCE-MIR-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-107-HAKO-MIMALLOC-POST-SMALL-ALLOC-DIRECT-SELECT-KEEPER-MEASUREMENT.md
---

# 296x-108 Hako Mimalloc Post Small-Alloc Direct Select Source/MIR Refresh

## Purpose

Refresh source/MIR and hot-owner rank after row107. The next selection should
account for the accepted small-alloc direct select keeper and the row101
rejected field shortcut.

## Required Output

```text
output_contract=hako-mimalloc-post-small-alloc-direct-select-source-mir-refresh-v0
input_contract=hako-mimalloc-post-small-alloc-direct-select-keeper-measurement-v0
method_count
active_method_count
rejected_keeper=select_single_page_active_field_fast_path
accepted_keeper=small_alloc_direct_single_page_select_fast_path
selected_owner
next_keeper
summary=ok
```

## Stop Line

Do not implement another keeper in this refresh row. Keep provider activation,
replacement, hooks, globals, and winner claims closed.

## Landed Evidence

```text
output_contract=hako-mimalloc-post-small-alloc-direct-select-source-mir-refresh-v0
input_contract=hako-mimalloc-post-small-alloc-direct-select-keeper-measurement-v0
method_count=4
active_method_count=4
inactive_surface_count=0
rejected_keeper=select_single_page_active_field_fast_path
rejected_keeper_reason=measured_regression_row102
accepted_keeper=small_alloc_direct_single_page_select_fast_path
active_method_rank_0=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
active_method_rank_0_source_method=objectLifecycleSmallAlloc
active_method_rank_0_active_count=524288
active_method_rank_0_mir_call_count=26
active_method_rank_0_mir_field_access_count=13
active_method_rank_0_mir_array_access_count=0
active_method_rank_0_score=47710208
active_method_rank_0_risk_kind=method_call_surface
active_method_rank_1=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2
active_method_rank_1_source_method=objectLifecycleReleaseBlock
active_method_rank_1_active_count=524288
active_method_rank_1_mir_call_count=22
active_method_rank_1_mir_field_access_count=4
active_method_rank_1_mir_array_access_count=1
active_method_rank_1_score=38797312
active_method_rank_1_risk_kind=method_call_surface
selected_owner=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
selected_source_method=objectLifecycleSmallAlloc
selected_risk_kind=method_call_surface
selected_reason=top_active_owner_still_small_alloc_method_call_surface_after_direct_select
selected_next_kind=keeper
next_keeper=small_alloc_inline_success_result_fast_path
next_keeper_kind=box_count
confidence=medium
next_row=HAKO-MIMALLOC-SMALL-ALLOC-INLINE-SUCCESS-RESULT-FAST-PATH-KEEPER-296X-001
winner_claim=0
replacement_active=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_post_small_alloc_direct_select_source_mir_refresh_guard.sh
```
