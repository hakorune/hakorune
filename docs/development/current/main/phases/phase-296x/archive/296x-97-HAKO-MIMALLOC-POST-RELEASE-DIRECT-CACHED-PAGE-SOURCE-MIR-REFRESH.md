---
Status: Landed
Date: 2026-05-27
Scope: refresh source/MIR observation after the release direct cached-page keeper.
Blocker: HAKO-MIMALLOC-POST-RELEASE-DIRECT-CACHED-PAGE-SOURCE-MIR-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-96-HAKO-MIMALLOC-POST-RELEASE-DIRECT-CACHED-PAGE-KEEPER-MEASUREMENT.md
  - docs/development/current/main/design/hako-check-mir-observation-boundary-ssot.md
---

# 296x-97 Hako Mimalloc Post Release Direct Cached-Page Source/MIR Refresh

## Purpose

Refresh source/MIR observation after row95. The next action should be selected
from current method shape and exact-EXE counters.

## Required Output

```text
output_contract=hako-mimalloc-post-release-direct-cached-page-source-mir-refresh-v0
input_contract=hako-mimalloc-post-release-direct-cached-page-keeper-measurement-v0
method_count
confirmed_source_mir_risk_count
selected_method
selected_risk_kind
next_keeper
summary=ok
```

## Stop Line

Do not implement another keeper in this refresh row.

## Landed Evidence

```text
output_contract=hako-mimalloc-post-release-direct-cached-page-source-mir-refresh-v0
input_contract=hako-mimalloc-post-release-direct-cached-page-keeper-measurement-v0
method_count=3
confirmed_source_mir_risk_count=3
select_page_single_fast_path_count=524288
select_page_single_fallback_count=0
selected_reason=single_page_select_hot_path_fallback_inactive
selected_method=HakoAllocObjectLifecyclePageQueue.selectPage/0
selected_source_method=selectPage
selected_hot_context=direct_loop
selected_risk_kind=array_access
next_keeper=select_single_page_first_page_cache
next_keeper_kind=box_count
next_row=HAKO-MIMALLOC-SELECT-SINGLE-PAGE-FIRST-PAGE-CACHE-KEEPER-296X-001
winner_claim=0
replacement_active=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_post_release_direct_cached_page_source_mir_refresh_guard.sh
```
