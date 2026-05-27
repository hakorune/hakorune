---
Status: Current
Date: 2026-05-27
Scope: rerun the in-process small-block measurement after the release known-page fast path keeper.
Blocker: HAKO-MIMALLOC-PERF-POST-RELEASE-KEEPER-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-78-HAKO-MIMALLOC-PERF-RELEASE-KNOWN-PAGE-FAST-PATH.md
  - tools/allocator/hako_mimalloc_in_process_operation_repeat_pilot.py
---

# 296x-79 Hako Mimalloc Perf Post-Release Keeper Measurement

## Purpose

Refresh the 8192-repeat in-process small-block evidence after the release
known-page fast path keeper.

## Required Output

```text
output_contract=hako-mimalloc-perf-post-release-keeper-measurement-v0
input_contract=hako-mimalloc-perf-release-known-page-fast-path-v0
operation_repeat=8192
sample_count
after_hako_elapsed_median_ms
previous_checkpoint_hako_elapsed_median_ms=240
keeper=release_known_page_fast_path
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

This row measures only. Next keeper selection is row 80.
