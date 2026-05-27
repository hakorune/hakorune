---
Status: Landed
Date: 2026-05-27
Scope: refresh source/MIR observation after the release known-page object cache keeper.
Blocker: HAKO-MIMALLOC-POST-RELEASE-OBJECT-CACHE-SOURCE-MIR-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-93-HAKO-MIMALLOC-POST-RELEASE-OBJECT-CACHE-KEEPER-MEASUREMENT.md
  - docs/development/current/main/design/hako-check-mir-observation-boundary-ssot.md
---

# 296x-94 Hako Mimalloc Post Release Object Cache Source/MIR Refresh

## Purpose

Refresh source/MIR observation after row92. The next decision should use the
current method shape after both selected-page caches have landed.

## Required Output

```text
output_contract=hako-mimalloc-post-release-object-cache-source-mir-refresh-v0
input_contract=hako-mimalloc-post-release-object-cache-keeper-measurement-v0
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
output_contract=hako-mimalloc-post-release-object-cache-source-mir-refresh-v0
input_contract=hako-mimalloc-post-release-object-cache-keeper-measurement-v0
method_count=3
confirmed_source_mir_risk_count=3
select_page_single_fallback_count=0
release_known_page_fast_path_count=524288
release_known_page_fallback_count=0
selected_reason=release_cache_hot_path_fallback_inactive
selected_method=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2
selected_source_method=objectLifecycleReleaseBlock
selected_hot_context=caller_repeated
selected_risk_kind=array_access
next_keeper=release_direct_cached_page_fast_path
next_keeper_kind=box_count
next_row=HAKO-MIMALLOC-RELEASE-DIRECT-CACHED-PAGE-FAST-PATH-KEEPER-296X-001
winner_claim=0
replacement_active=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_post_release_object_cache_source_mir_refresh_guard.sh
```
