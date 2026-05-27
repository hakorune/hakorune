---
Status: Landed
Date: 2026-05-27
Scope: refresh source/MIR observation after the select first-page cache keeper measurement.
Blocker: HAKO-MIMALLOC-POST-SELECT-FIRST-PAGE-CACHE-SOURCE-MIR-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-99-HAKO-MIMALLOC-POST-SELECT-FIRST-PAGE-CACHE-KEEPER-MEASUREMENT.md
  - docs/development/current/main/design/hako-check-mir-observation-boundary-ssot.md
---

# 296x-100 Hako Mimalloc Post Select First-Page Cache Source/MIR Refresh

## Purpose

Refresh source/MIR observation after row99. The next action should be selected
from current method shape and exact-EXE counters, not from stale row97 risk.

## Required Output

```text
output_contract=hako-mimalloc-post-select-first-page-cache-source-mir-refresh-v0
input_contract=hako-mimalloc-post-select-first-page-cache-keeper-measurement-v0
method_count
confirmed_source_mir_risk_count
selected_method
selected_risk_kind
next_keeper
summary=ok
```

## Stop Line

Do not implement another keeper in this refresh row. Keep provider activation,
replacement, hooks, globals, and winner claims closed.

## Landed Evidence

```text
output_contract=hako-mimalloc-post-select-first-page-cache-source-mir-refresh-v0
input_contract=hako-mimalloc-post-select-first-page-cache-keeper-measurement-v0
method_count=6
confirmed_source_mir_risk_count=6
select_page_single_fast_path_count=524288
select_page_single_fallback_count=0
release_known_page_fast_path_count=524288
release_known_page_fallback_count=0
inactive_select_loop_risk=1
inactive_release_lookup_risk=1
selected_reason=active_single_page_select_field_method_surface
selected_method=HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0
selected_source_method=selectSinglePageFastPath
selected_hot_context=caller_repeated
selected_risk_kind=field_access
selected_mir_call_count=6
selected_mir_field_access_count=20
next_keeper=select_single_page_active_field_fast_path
next_keeper_kind=box_count
next_row=HAKO-MIMALLOC-SELECT-SINGLE-PAGE-ACTIVE-FIELD-FAST-PATH-KEEPER-296X-001
winner_claim=0
replacement_active=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_post_select_first_page_cache_source_mir_refresh_guard.sh
```
