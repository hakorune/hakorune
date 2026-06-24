---
Status: Landed
Date: 2026-05-27
Scope: bypass the selectPage wrapper from small alloc when the workload is single-page.
Blocker: HAKO-MIMALLOC-SMALL-ALLOC-DIRECT-SINGLE-PAGE-SELECT-FAST-PATH-KEEPER-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-105-HAKO-MIMALLOC-POST-ROLLBACK-SOURCE-MIR-REFRESH.md
---

# 296x-106 Hako Mimalloc Small-Alloc Direct Single-Page Select Fast Path Keeper

## Purpose

Apply one BoxCount keeper selected by row105:

```text
keeper=small_alloc_direct_single_page_select_fast_path
keeper_kind=box_count
```

The hot-owner rank selected `objectLifecycleSmallAlloc/1` as the top active
owner. The workload has `select_page_single_fallback_count=0`, so a narrow
single-page branch can bypass the `selectPage()` wrapper and call the
single-page select route directly while preserving the generic `selectPage()`
fallback for multi-page cases.

## Required Output

```text
output_contract=hako-mimalloc-small-alloc-direct-single-page-select-fast-path-keeper-v0
input_contract=hako-mimalloc-hot-owner-rank-v0
keeper=small_alloc_direct_single_page_select_fast_path
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
direct_single_page_select_used=1
generic_select_page_fallback_preserved=1
proof_summary=ok
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

Do not remove the generic multi-page `selectPage()` fallback. Do not change
release behavior, provider activation, replacement, hooks, globals, or winner
claims in this row.

## Landed Evidence

```text
output_contract=hako-mimalloc-small-alloc-direct-single-page-select-fast-path-keeper-v0
input_contract=hako-mimalloc-hot-owner-rank-v0
keeper=small_alloc_direct_single_page_select_fast_path
keeper_kind=box_count
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
direct_single_page_select_used=1
generic_select_page_fallback_preserved=1
proof_app=apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako
proof_summary=ok
select_page_single_fast_path_count=524288
select_page_single_fallback_count=0
release_known_page_fast_path_count=524288
winner_claim=0
replacement_active=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_small_alloc_direct_single_page_select_fast_path_keeper_guard.sh
```
