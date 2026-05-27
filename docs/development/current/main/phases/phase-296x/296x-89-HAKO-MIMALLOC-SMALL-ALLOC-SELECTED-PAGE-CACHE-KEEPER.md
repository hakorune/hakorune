---
Status: Landed
Date: 2026-05-27
Scope: apply the selected small-alloc page-cache keeper without widening allocator activation or winner claims.
Blocker: HAKO-MIMALLOC-SMALL-ALLOC-SELECTED-PAGE-CACHE-KEEPER-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-88-HAKO-MIMALLOC-MULTI-METHOD-SOURCE-MIR-OBSERVATION.md
  - docs/development/current/main/design/hako-check-mir-observation-boundary-ssot.md
---

# 296x-89 Hako Mimalloc Small-Alloc Selected Page Cache Keeper

## Purpose

Apply one narrow `.hako` keeper selected by row88:

```text
keeper=small_alloc_selected_page_cache_reuse
keeper_kind=box_count
```

`HakoAllocObjectLifecyclePageQueue.selectPage()` already accepts and records the
selected page. `HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1`
currently performs another `pages.get(selected_index)` in a caller-repeated hot
method. Cache the accepted page on the queue and reuse it from the facade.

## Required Output

```text
output_contract=hako-mimalloc-small-alloc-selected-page-cache-keeper-v0
input_contract=hako-mimalloc-multi-method-source-mir-observation-v0
keeper
target_method
selected_page_cache_reused=1
proof_summary=ok
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

Do not change release lookup, multi-page selection policy, provider activation,
process replacement, hooks, globals, or winner claims in this row.

## Landed Evidence

```text
output_contract=hako-mimalloc-small-alloc-selected-page-cache-keeper-v0
input_contract=hako-mimalloc-multi-method-source-mir-observation-v0
keeper=small_alloc_selected_page_cache_reuse
keeper_kind=box_count
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
selected_page_cache_reused=1
removed_repeated_pages_get=1
proof_app=apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako
proof_summary=ok
select_page_single_fast_path_count=524288
release_known_page_fast_path_count=524288
source_array_access_count_after=0
mir_array_get_call_count_after=0
winner_claim=0
replacement_active=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_small_alloc_selected_page_cache_keeper_guard.sh
```
