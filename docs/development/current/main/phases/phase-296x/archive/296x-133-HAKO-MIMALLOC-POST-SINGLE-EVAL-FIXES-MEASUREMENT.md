---
Status: Landed
Date: 2026-05-28
Scope: measure object-lifecycle facade exact-EXE after nested argument/field/env single-eval fixes.
Blocker: HAKO-MIMALLOC-POST-SINGLE-EVAL-FIXES-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-128-MIR-BUILDER-NESTED-ARGUMENT-SINGLE-EVAL-OWNER-FIX.md
  - docs/development/current/main/phases/phase-296x/296x-130-MIR-BUILDER-NESTED-FIELD-SINGLE-EVAL-OWNER-FIX.md
  - docs/development/current/main/phases/phase-296x/296x-132-MIR-BUILDER-ENV-METHOD-SINGLE-EVAL-OWNER-FIX.md
---

# 296x-133 Hako Mimalloc Post Single Eval Fixes Measurement

## Purpose

Measure the object-lifecycle facade exact-EXE after the compiler
single-evaluation correctness fixes are closed.

## Required Output

```text
output_contract=hako-mimalloc-post-single-eval-fixes-measurement-v0
input_contract=mir-builder-env-method-single-eval-owner-fix-v0
measurement_profile=object_lifecycle_facade_exact_exe
sample_count
elapsed_median_ms
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

Do not add static scalar method fact inference or lowering in this row.

## Evidence

Report:

```text
output_contract=hako-mimalloc-post-single-eval-fixes-measurement-v0
input_contract=mir-builder-env-method-single-eval-owner-fix-v0
measurement_profile=object_lifecycle_facade_exact_exe
measurement_scope=object_lifecycle_facade_exact_exe_after_single_eval_fixes
workload_id=representative-object-lifecycle-small-block-v0
operation_repeat=8192
sample_count=3
select_page_single_fast_path_count=524288
release_known_page_fast_path_count=524288
sample_0_hako_external_elapsed_ms=600
sample_1_hako_external_elapsed_ms=610
sample_2_hako_external_elapsed_ms=610
elapsed_median_ms=610
elapsed_min_ms=600
elapsed_max_ms=610
external_rss_median_bytes=3657728
previous_checkpoint_hako_elapsed_median_ms=610
previous_checkpoint_source=296x-124-post-hako-reason-bind-measurement
single_eval_fix_effect=neutral
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_post_single_eval_fixes_measurement_guard.sh
```
