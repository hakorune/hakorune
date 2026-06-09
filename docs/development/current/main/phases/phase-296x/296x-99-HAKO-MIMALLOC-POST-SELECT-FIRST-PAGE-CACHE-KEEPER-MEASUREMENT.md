---
Status: Landed
Date: 2026-05-27
Scope: measure the object-lifecycle facade exact-EXE after the select first-page cache keeper.
Blocker: HAKO-MIMALLOC-POST-SELECT-FIRST-PAGE-CACHE-KEEPER-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-98-HAKO-MIMALLOC-SELECT-SINGLE-PAGE-FIRST-PAGE-CACHE-KEEPER.md
---

# 296x-99 Hako Mimalloc Post Select First-Page Cache Keeper Measurement

## Purpose

Rerun the repeated exact-EXE measurement after row98 so the next optimization
choice is based on measured behavior, not the keeper proof alone.

## Required Output

```text
output_contract=hako-mimalloc-post-select-first-page-cache-keeper-measurement-v0
input_contract=hako-mimalloc-select-single-page-first-page-cache-keeper-v0
measurement_scope=object_lifecycle_facade_exact_exe_after_select_first_page_cache_keeper
keeper=select_single_page_first_page_cache
sample_count
elapsed_median_ms
select_page_single_fast_path_count
select_page_single_fallback_count=0
release_known_page_fast_path_count
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

Do not implement another keeper, open provider activation, replacement, hooks,
globals, or winner claims in this row.

## Landed Evidence

```text
output_contract=hako-mimalloc-post-select-first-page-cache-keeper-measurement-v0
input_contract=hako-mimalloc-select-single-page-first-page-cache-keeper-v0
measurement_scope=object_lifecycle_facade_exact_exe_after_select_first_page_cache_keeper
workload_id=representative-object-lifecycle-small-block-v0
operation_repeat=8192
sample_count=1
keeper=select_single_page_first_page_cache
select_page_single_fast_path_count=524288
select_page_single_fallback_count=0
release_known_page_fast_path_count=524288
release_known_page_fallback_count=0
sample_0_hako_external_elapsed_ms=350
after_hako_elapsed_median_ms=350
after_hako_elapsed_min_ms=350
after_hako_elapsed_max_ms=350
after_hako_external_rss_median_bytes=3633152
previous_keeper=release_direct_cached_page_fast_path
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_post_select_first_page_cache_keeper_measurement_guard.sh
```
