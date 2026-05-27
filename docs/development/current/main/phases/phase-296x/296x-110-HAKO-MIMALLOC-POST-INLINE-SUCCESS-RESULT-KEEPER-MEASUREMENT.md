---
Status: Landed
Date: 2026-05-27
Scope: measure the object-lifecycle facade exact-EXE after the inline success result keeper.
Blocker: HAKO-MIMALLOC-POST-INLINE-SUCCESS-RESULT-KEEPER-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-109-HAKO-MIMALLOC-SMALL-ALLOC-INLINE-SUCCESS-RESULT-FAST-PATH-KEEPER.md
---

# 296x-110 Hako Mimalloc Post Inline Success Result Keeper Measurement

## Purpose

Rerun the repeated exact-EXE measurement after row109 so the inline success
result keeper is judged by measured behavior.

## Required Output

```text
output_contract=hako-mimalloc-post-inline-success-result-keeper-measurement-v0
input_contract=hako-mimalloc-small-alloc-inline-success-result-fast-path-keeper-v0
measurement_scope=object_lifecycle_facade_exact_exe_after_inline_success_result_keeper
keeper=small_alloc_inline_success_result_fast_path
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
output_contract=hako-mimalloc-post-inline-success-result-keeper-measurement-v0
input_contract=hako-mimalloc-small-alloc-inline-success-result-fast-path-keeper-v0
measurement_scope=object_lifecycle_facade_exact_exe_after_inline_success_result_keeper
workload_id=representative-object-lifecycle-small-block-v0
operation_repeat=8192
sample_count=3
keeper=small_alloc_inline_success_result_fast_path
select_page_single_fast_path_count=524288
select_page_single_fallback_count=0
release_known_page_fast_path_count=524288
release_known_page_fallback_count=0
sample_0_hako_external_elapsed_ms=650
sample_1_hako_external_elapsed_ms=620
sample_2_hako_external_elapsed_ms=630
after_hako_elapsed_median_ms=630
after_hako_elapsed_min_ms=620
after_hako_elapsed_max_ms=650
after_hako_external_rss_median_bytes=3559424
previous_checkpoint_median_ms=620
median_delta_vs_previous_ms=10
keeper_effect=regressed
next_row=HAKO-MIMALLOC-ROLLBACK-INLINE-SUCCESS-RESULT-KEEPER-296X-001
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_post_inline_success_result_keeper_measurement_guard.sh
```
