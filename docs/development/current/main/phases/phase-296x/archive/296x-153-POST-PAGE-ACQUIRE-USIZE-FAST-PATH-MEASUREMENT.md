---
Status: Landed
Date: 2026-05-28
Scope: measure exact-EXE after the small-alloc acquire_usize fast path keeper.
Blocker: POST-PAGE-ACQUIRE-USIZE-FAST-PATH-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-152-SMALL-ALLOC-PAGE-ACQUIRE-USIZE-FAST-PATH-IMPLEMENTATION.md
---

# 296x-153 Post Page Acquire Usize Fast Path Measurement

## Purpose

Run the exact-EXE scout measurement after the small-alloc `acquire_usize`
keeper, then classify whether the keeper is accepted, neutral, or regressed.

## Required Output

```text
output_contract=post-page-acquire-usize-fast-path-measurement-v0
input_contract=small-alloc-page-acquire-usize-fast-path-implementation-v0
elapsed_median_ms
previous_checkpoint_ms=600
keeper_effect
winner_claim=0
replacement_active=0
summary=ok
```

## Evidence

```text
output_contract=post-page-acquire-usize-fast-path-measurement-v0
input_contract=small-alloc-page-acquire-usize-fast-path-implementation-v0
measurement_scope=object_lifecycle_facade_exact_exe_after_acquire_usize_fast_path
workload_id=representative-object-lifecycle-small-block-v0
operation_repeat=8192
sample_count=1
release_known_page_fast_path_count=524288
release_known_page_fallback_count=0
sample_0_hako_external_elapsed_ms=570
elapsed_median_ms=570
elapsed_min_ms=570
elapsed_max_ms=570
external_rss_median_bytes=3649536
previous_checkpoint_median_ms=600
previous_checkpoint_source=296x-149-post-known-live-release-measurement
keeper_effect=accepted
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
selected_next=post_page_acquire_usize_source_mir_refresh
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_post_page_acquire_usize_fast_path_measurement_guard.sh
```
