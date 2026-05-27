---
Status: Landed
Date: 2026-05-27
Scope: measure the object-lifecycle facade exact-EXE after the release direct cached-page fast path keeper.
Blocker: HAKO-MIMALLOC-POST-RELEASE-DIRECT-CACHED-PAGE-KEEPER-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-95-HAKO-MIMALLOC-RELEASE-DIRECT-CACHED-PAGE-FAST-PATH-KEEPER.md
---

# 296x-96 Hako Mimalloc Post Release Direct Cached-Page Keeper Measurement

## Purpose

Rerun the object-lifecycle facade exact-EXE measurement after row95. Preserve
semantic counters and keep winner/replacement claims closed.

## Required Output

```text
output_contract=hako-mimalloc-post-release-direct-cached-page-keeper-measurement-v0
input_contract=hako-mimalloc-release-direct-cached-page-fast-path-keeper-v0
measurement_scope=object_lifecycle_facade_exact_exe_after_release_direct_cached_page_keeper
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
output_contract=hako-mimalloc-post-release-direct-cached-page-keeper-measurement-v0
input_contract=hako-mimalloc-release-direct-cached-page-fast-path-keeper-v0
measurement_scope=object_lifecycle_facade_exact_exe_after_release_direct_cached_page_keeper
workload_id=representative-object-lifecycle-small-block-v0
operation_repeat=8192
sample_count=3
keeper=release_direct_cached_page_fast_path
select_page_single_fast_path_count=524288
select_page_single_fallback_count=0
release_known_page_fast_path_count=524288
release_known_page_fallback_count=0
sample_0_hako_external_elapsed_ms=650
sample_1_hako_external_elapsed_ms=650
sample_2_hako_external_elapsed_ms=650
after_hako_elapsed_median_ms=650
after_hako_elapsed_min_ms=650
after_hako_elapsed_max_ms=650
after_hako_external_rss_median_bytes=3588096
previous_median_ms=670
median_delta_ms=-20
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_post_release_direct_cached_page_keeper_measurement_guard.sh
```
