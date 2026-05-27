---
Status: Landed
Date: 2026-05-27
Scope: refresh source/MIR observation after the small-alloc selected-page cache keeper before selecting another keeper.
Blocker: HAKO-MIMALLOC-POST-SMALL-ALLOC-CACHE-SOURCE-MIR-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-90-HAKO-MIMALLOC-POST-SMALL-ALLOC-CACHE-KEEPER-MEASUREMENT.md
  - docs/development/current/main/design/hako-check-mir-observation-boundary-ssot.md
---

# 296x-91 Hako Mimalloc Post Small-Alloc Cache Source/MIR Refresh

## Purpose

Refresh source/MIR observation after row89 removed the caller-repeated
`pages.get(selected_index)` from `objectLifecycleSmallAlloc/1`.

The next selection must be based on current source/MIR shape, not the pre-row89
array access evidence.

## Required Output

```text
output_contract=hako-mimalloc-post-small-alloc-cache-source-mir-refresh-v0
input_contract=hako-mimalloc-post-small-alloc-cache-keeper-measurement-v0
method_count
confirmed_source_mir_risk_count
selected_method
selected_risk_kind
next_keeper
summary=ok
```

## Stop Line

Do not implement the next keeper in this refresh row.

## Landed Evidence

```text
output_contract=hako-mimalloc-post-small-alloc-cache-source-mir-refresh-v0
input_contract=hako-mimalloc-post-small-alloc-cache-keeper-measurement-v0
method_count=3
confirmed_source_mir_risk_count=3
select_page_single_fallback_count=0
release_known_page_fast_path_count=524288
inactive_risk=select_page_loop_inactive_for_single_page_workload
selected_method=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2
selected_source_method=objectLifecycleReleaseBlock
selected_hot_context=caller_repeated
selected_risk_kind=array_access
next_keeper=release_known_page_object_cache
next_keeper_kind=box_shape
next_row=HAKO-MIMALLOC-RELEASE-KNOWN-PAGE-OBJECT-CACHE-KEEPER-296X-001
winner_claim=0
replacement_active=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_post_small_alloc_cache_source_mir_refresh_guard.sh
```
