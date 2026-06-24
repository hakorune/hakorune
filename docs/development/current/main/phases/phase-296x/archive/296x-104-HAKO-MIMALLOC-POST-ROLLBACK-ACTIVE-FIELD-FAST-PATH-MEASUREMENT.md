---
Status: Landed
Date: 2026-05-27
Scope: measure the object-lifecycle facade exact-EXE after rolling back the active field fast path.
Blocker: HAKO-MIMALLOC-POST-ROLLBACK-ACTIVE-FIELD-FAST-PATH-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-103-HAKO-MIMALLOC-ROLLBACK-ACTIVE-FIELD-FAST-PATH-KEEPER.md
---

# 296x-104 Hako Mimalloc Post Rollback Active Field Fast Path Measurement

## Purpose

Rerun the repeated exact-EXE measurement after row103 to confirm the source has
returned to the row99 first-page cache baseline before selecting another
optimization.

## Required Output

```text
output_contract=hako-mimalloc-post-rollback-active-field-fast-path-measurement-v0
input_contract=hako-mimalloc-rollback-active-field-fast-path-keeper-v0
measurement_scope=object_lifecycle_facade_exact_exe_after_active_field_fast_path_rollback
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
output_contract=hako-mimalloc-post-rollback-active-field-fast-path-measurement-v0
input_contract=hako-mimalloc-rollback-active-field-fast-path-keeper-v0
measurement_scope=object_lifecycle_facade_exact_exe_after_active_field_fast_path_rollback
workload_id=representative-object-lifecycle-small-block-v0
operation_repeat=8192
sample_count=3
rolled_back_keeper=select_single_page_active_field_fast_path
select_page_single_fast_path_count=524288
select_page_single_fallback_count=0
release_known_page_fast_path_count=524288
release_known_page_fallback_count=0
sample_0_hako_external_elapsed_ms=640
sample_1_hako_external_elapsed_ms=630
sample_2_hako_external_elapsed_ms=640
after_hako_elapsed_median_ms=640
after_hako_elapsed_min_ms=630
after_hako_elapsed_max_ms=640
after_hako_external_rss_median_bytes=3534848
baseline_median_ms=620
regressed_median_ms=650
rollback_delta_vs_regressed_ms=-10
rollback_delta_vs_baseline_ms=20
rollback_confirmed=1
baseline_exact_reproduced=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_post_rollback_active_field_fast_path_measurement_guard.sh
```
