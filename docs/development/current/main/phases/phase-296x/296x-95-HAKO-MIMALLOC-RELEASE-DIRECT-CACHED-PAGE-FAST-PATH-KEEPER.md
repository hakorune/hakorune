---
Status: Landed
Date: 2026-05-27
Scope: apply a direct cached-page release fast path for the object-lifecycle facade.
Blocker: HAKO-MIMALLOC-RELEASE-DIRECT-CACHED-PAGE-FAST-PATH-KEEPER-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-94-HAKO-MIMALLOC-POST-RELEASE-OBJECT-CACHE-SOURCE-MIR-REFRESH.md
---

# 296x-95 Hako Mimalloc Release Direct Cached Page Fast Path Keeper

## Purpose

Apply one BoxCount keeper selected by row94:

```text
keeper=release_direct_cached_page_fast_path
keeper_kind=box_count
```

The current workload always releases the last allocated page through the known
page fast path. Bypass the index-return helper on that hot route by releasing
directly through the cached page object, while preserving the existing fallback
lookup for non-hot cases.

## Required Output

```text
output_contract=hako-mimalloc-release-direct-cached-page-fast-path-keeper-v0
input_contract=hako-mimalloc-post-release-object-cache-source-mir-refresh-v0
keeper
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2
direct_cached_page_release_fast_path=1
fallback_lookup_preserved=1
proof_summary=ok
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

Do not change free ordering, provider activation, replacement, hooks, globals,
or winner claims in this row.

## Landed Evidence

```text
output_contract=hako-mimalloc-release-direct-cached-page-fast-path-keeper-v0
input_contract=hako-mimalloc-post-release-object-cache-source-mir-refresh-v0
keeper=release_direct_cached_page_fast_path
keeper_kind=box_count
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2
direct_cached_page_release_fast_path=1
fallback_lookup_preserved=1
proof_app=apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako
proof_summary=ok
select_page_single_fast_path_count=524288
release_known_page_fast_path_count=524288
release_known_page_fallback_count=0
winner_claim=0
replacement_active=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_release_direct_cached_page_fast_path_keeper_guard.sh
```
