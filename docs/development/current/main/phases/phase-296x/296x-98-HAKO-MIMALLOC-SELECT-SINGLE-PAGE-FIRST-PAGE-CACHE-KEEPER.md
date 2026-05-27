---
Status: Current
Date: 2026-05-27
Scope: cache the first page object for the single-page select hot path.
Blocker: HAKO-MIMALLOC-SELECT-SINGLE-PAGE-FIRST-PAGE-CACHE-KEEPER-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-97-HAKO-MIMALLOC-POST-RELEASE-DIRECT-CACHED-PAGE-SOURCE-MIR-REFRESH.md
---

# 296x-98 Hako Mimalloc Select Single-Page First-Page Cache Keeper

## Purpose

Apply one BoxCount keeper selected by row97:

```text
keeper=select_single_page_first_page_cache
keeper_kind=box_count
```

The selected workload always uses the single-page select fast path. Cache the
first page object at `addPage` time and let `selectSinglePageFastPath()` read
that object instead of calling `pages.get(0)` on the hot route.

## Required Output

```text
output_contract=hako-mimalloc-select-single-page-first-page-cache-keeper-v0
input_contract=hako-mimalloc-post-release-direct-cached-page-source-mir-refresh-v0
keeper
target_method=HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0
first_page_cache_used=1
proof_summary=ok
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

Do not change multi-page selection policy, provider activation, replacement,
hooks, globals, or winner claims in this row.
