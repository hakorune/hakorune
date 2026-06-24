---
Status: Landed
Date: 2026-05-27
Scope: measure the object-lifecycle facade exact-EXE after the release known-page object cache keeper.
Blocker: HAKO-MIMALLOC-POST-RELEASE-OBJECT-CACHE-KEEPER-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-92-HAKO-MIMALLOC-RELEASE-KNOWN-PAGE-OBJECT-CACHE-KEEPER.md
---

# 296x-93 Hako Mimalloc Post Release Object Cache Keeper Measurement

## Purpose

Rerun the object-lifecycle facade exact-EXE measurement after row92. Preserve
semantic counters and keep winner/replacement claims closed.

## Required Output

```text
output_contract=hako-mimalloc-post-release-object-cache-keeper-measurement-v0
input_contract=hako-mimalloc-release-known-page-object-cache-keeper-v0
measurement_scope=object_lifecycle_facade_exact_exe_after_release_object_cache_keeper
workload_id=representative-object-lifecycle-small-block-v0
operation_repeat=8192
sample_count
after_hako_elapsed_median_ms
select_page_single_fast_path_count=524288
release_known_page_fast_path_count=524288
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

Do not implement another keeper in this measurement row.

## Landed Evidence

```text
output_contract=hako-mimalloc-post-release-object-cache-keeper-measurement-v0
input_contract=hako-mimalloc-release-known-page-object-cache-keeper-v0
measurement_scope=object_lifecycle_facade_exact_exe_after_release_object_cache_keeper
workload_id=representative-object-lifecycle-small-block-v0
operation_repeat=8192
sample_count=3
keeper=release_known_page_object_cache
select_page_single_fast_path_count=524288
select_page_single_fallback_count=0
release_known_page_fast_path_count=524288
release_known_page_fallback_count=0
sample_0_hako_external_elapsed_ms=670
sample_1_hako_external_elapsed_ms=700
sample_2_hako_external_elapsed_ms=670
after_hako_elapsed_median_ms=670
after_hako_elapsed_min_ms=670
after_hako_elapsed_max_ms=700
after_hako_external_rss_median_bytes=3657728
previous_median_ms=690
median_delta_ms=-20
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_post_release_object_cache_keeper_measurement_guard.sh
```
