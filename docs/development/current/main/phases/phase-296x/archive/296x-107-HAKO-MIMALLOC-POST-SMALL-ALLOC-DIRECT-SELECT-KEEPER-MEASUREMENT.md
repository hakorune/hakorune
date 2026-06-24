---
Status: Landed
Date: 2026-05-27
Scope: measure the object-lifecycle facade exact-EXE after the small-alloc direct select keeper.
Blocker: HAKO-MIMALLOC-POST-SMALL-ALLOC-DIRECT-SELECT-KEEPER-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-106-HAKO-MIMALLOC-SMALL-ALLOC-DIRECT-SINGLE-PAGE-SELECT-FAST-PATH-KEEPER.md
---

# 296x-107 Hako Mimalloc Post Small-Alloc Direct Select Keeper Measurement

## Purpose

Rerun the repeated exact-EXE measurement after row106 so the direct single-page
select keeper is judged by measured behavior.

## Required Output

```text
output_contract=hako-mimalloc-post-small-alloc-direct-select-keeper-measurement-v0
input_contract=hako-mimalloc-small-alloc-direct-single-page-select-fast-path-keeper-v0
measurement_scope=object_lifecycle_facade_exact_exe_after_small_alloc_direct_select_keeper
keeper=small_alloc_direct_single_page_select_fast_path
sample_count
after_hako_elapsed_median_ms
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
output_contract=hako-mimalloc-post-small-alloc-direct-select-keeper-measurement-v0
input_contract=hako-mimalloc-small-alloc-direct-single-page-select-fast-path-keeper-v0
measurement_scope=object_lifecycle_facade_exact_exe_after_small_alloc_direct_select_keeper
workload_id=representative-object-lifecycle-small-block-v0
operation_repeat=8192
sample_count=3
keeper=small_alloc_direct_single_page_select_fast_path
select_page_single_fast_path_count=524288
select_page_single_fallback_count=0
release_known_page_fast_path_count=524288
release_known_page_fallback_count=0
sample_0_hako_external_elapsed_ms=620
sample_1_hako_external_elapsed_ms=610
sample_2_hako_external_elapsed_ms=620
after_hako_elapsed_median_ms=620
after_hako_elapsed_min_ms=610
after_hako_elapsed_max_ms=620
after_hako_external_rss_median_bytes=3604480
previous_checkpoint_median_ms=640
best_checkpoint_median_ms=620
median_delta_vs_previous_ms=-20
keeper_effect=accepted
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_post_small_alloc_direct_select_keeper_measurement_guard.sh
```
