---
Status: Current
Date: 2026-05-27
Scope: apply the selected release known-page object cache keeper without changing release semantics.
Blocker: HAKO-MIMALLOC-RELEASE-KNOWN-PAGE-OBJECT-CACHE-KEEPER-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-91-HAKO-MIMALLOC-POST-SMALL-ALLOC-CACHE-SOURCE-MIR-REFRESH.md
---

# 296x-92 Hako Mimalloc Release Known-Page Object Cache Keeper

## Purpose

Apply one BoxShape keeper selected by row91:

```text
keeper=release_known_page_object_cache
keeper_kind=box_shape
```

The release fast path currently validates the last allocation by index and then
does another `pages.get(known_index)` in `objectLifecycleReleaseBlock/2`. Cache
the last allocated page object alongside the existing last allocation index/id
and use that object for the known-page release path.

## Required Output

```text
output_contract=hako-mimalloc-release-known-page-object-cache-keeper-v0
input_contract=hako-mimalloc-post-small-alloc-cache-source-mir-refresh-v0
keeper
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2
release_known_page_object_cache_reused=1
proof_summary=ok
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

Do not change free ordering, page-map lookup, provider activation, replacement,
hooks, globals, or winner claims in this row.
