---
Status: Landed
Date: 2026-05-28
Scope: measure object-lifecycle facade exact-EXE after static-scalar call lowering.
Blocker: POST-STATIC-SCALAR-CALL-LOWERING-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-133-HAKO-MIMALLOC-POST-SINGLE-EVAL-FIXES-MEASUREMENT.md
  - docs/development/current/main/phases/phase-296x/296x-138-STATIC-SCALAR-CALL-LOWERING-IMPLEMENTATION.md
---

# 296x-139 Post Static Scalar Call Lowering Measurement

## Purpose

Rerun the object-lifecycle facade exact-EXE measurement after verified
static-scalar reason calls lower to constants.

## Required Output

```text
output_contract=post-static-scalar-call-lowering-measurement-v0
input_contract=static-scalar-call-lowering-implementation-v0
elapsed_median_ms
previous_checkpoint_hako_elapsed_median_ms
static_scalar_lowering_effect
winner_claim=0
summary=ok
```

## Evidence

Report:

```text
output_contract=post-static-scalar-call-lowering-measurement-v0
input_contract=static-scalar-call-lowering-implementation-v0
measurement_profile=object_lifecycle_facade_exact_exe
measurement_scope=object_lifecycle_facade_exact_exe_after_static_scalar_call_lowering
workload_id=representative-object-lifecycle-small-block-v0
operation_repeat=8192
sample_count=3
select_page_single_fast_path_count=524288
release_known_page_fast_path_count=524288
sample_0_hako_external_elapsed_ms=610
sample_1_hako_external_elapsed_ms=610
sample_2_hako_external_elapsed_ms=600
elapsed_median_ms=610
elapsed_min_ms=600
elapsed_max_ms=610
external_rss_median_bytes=3637248
previous_checkpoint_hako_elapsed_median_ms=610
previous_checkpoint_source=296x-133-post-single-eval-fixes-measurement
static_scalar_lowering_effect=neutral
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
selected_next=post_static_scalar_source_mir_refresh
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_post_static_scalar_call_lowering_measurement_guard.sh
```
