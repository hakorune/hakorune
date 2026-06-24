---
Status: Landed
Date: 2026-05-27
Scope: measure the object-lifecycle facade exact-EXE after rolling back the inline success result keeper.
Blocker: HAKO-MIMALLOC-POST-ROLLBACK-INLINE-SUCCESS-RESULT-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-111-HAKO-MIMALLOC-ROLLBACK-INLINE-SUCCESS-RESULT-KEEPER.md
---

# 296x-112 Hako Mimalloc Post Rollback Inline Success Result Measurement

## Purpose

Rerun the repeated exact-EXE measurement after row111 to confirm the source has
returned to the accepted direct select keeper baseline before selecting another
optimization.

## Required Output

```text
output_contract=hako-mimalloc-post-rollback-inline-success-result-measurement-v0
input_contract=hako-mimalloc-rollback-inline-success-result-keeper-v0
measurement_scope=object_lifecycle_facade_exact_exe_after_inline_success_result_rollback
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
output_contract=hako-mimalloc-post-rollback-inline-success-result-measurement-v0
input_contract=hako-mimalloc-rollback-inline-success-result-keeper-v0
measurement_scope=object_lifecycle_facade_exact_exe_after_inline_success_result_rollback
workload_id=representative-object-lifecycle-small-block-v0
operation_repeat=8192
sample_count=3
rolled_back_keeper=small_alloc_inline_success_result_fast_path
select_page_single_fast_path_count=524288
select_page_single_fallback_count=0
release_known_page_fast_path_count=524288
release_known_page_fallback_count=0
sample_0_hako_external_elapsed_ms=620
sample_1_hako_external_elapsed_ms=620
sample_2_hako_external_elapsed_ms=600
after_hako_elapsed_median_ms=620
after_hako_elapsed_min_ms=600
after_hako_elapsed_max_ms=620
after_hako_external_rss_median_bytes=3592192
baseline_median_ms=620
regressed_median_ms=630
rollback_confirmed=1
baseline_exact_reproduced=1
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_post_rollback_inline_success_result_measurement_guard.sh
```
