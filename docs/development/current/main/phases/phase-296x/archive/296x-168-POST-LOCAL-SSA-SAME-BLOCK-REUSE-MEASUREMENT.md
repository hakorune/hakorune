---
Status: Landed
Date: 2026-05-28
Scope: measure exact-EXE timing after LocalSSA same-block field_get reuse.
Blocker: POST-LOCAL-SSA-SAME-BLOCK-REUSE-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-167-LOCAL-SSA-SAME-BLOCK-REUSE-IMPLEMENTATION.md
  - tools/allocator/hako_mimalloc_post_local_ssa_same_block_reuse_measurement.py
---

# 296x-168 Post LocalSSA Same-Block Reuse Measurement

## Purpose

Measure exact-EXE timing after row167's field_get-only LocalSSA same-block
reuse. This row closes the implementation measurement before deciding whether
to keep or roll back the structural cleanup.

## Required Output

```text
output_contract=hako-mimalloc-post-local-ssa-same-block-reuse-measurement-v0
input_contract=local-ssa-same-block-reuse-implementation-v0
after_hako_elapsed_median_ms
measurement_effect
summary=ok
```

## Evidence

```text
output_contract=hako-mimalloc-post-local-ssa-same-block-reuse-measurement-v0
input_contract=local-ssa-same-block-reuse-implementation-v0
measurement_scope=object_lifecycle_facade_exact_exe_after_local_ssa_same_block_field_get_reuse
workload_id=representative-object-lifecycle-small-block-v0
operation_repeat=8192
sample_count=3
keeper=local_ssa_same_block_field_get_reuse
select_page_single_fast_path_count=524288
select_page_single_fallback_count=0
release_known_page_fast_path_count=524288
release_known_page_fallback_count=0
sample_0_hako_external_elapsed_ms=570
sample_1_hako_external_elapsed_ms=570
sample_2_hako_external_elapsed_ms=570
after_hako_elapsed_median_ms=570
after_hako_elapsed_min_ms=570
after_hako_elapsed_max_ms=570
after_hako_external_rss_median_bytes=3649536
previous_checkpoint_hako_elapsed_median_ms=550
delta_hako_elapsed_median_ms=20
measurement_effect=regressed
structural_keeper=1
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Interpretation:

```text
The row167 field_get-only cleanup improved MIR structure but regressed the
exact-EXE median against the row163 checkpoint. Treat this as a performance
non-keeper for the mimalloc parity lane and roll it back before selecting the
next owner.
```

## Next

```text
row169:
  rollback-local-ssa-same-block-reuse

Goal:
  remove the row167 LocalSSA same-block field_get reuse rule, restore the
  row163/row164 structural baseline, and keep the row165/row166 evidence as a
  failed-owner trail.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_post_local_ssa_same_block_reuse_measurement_guard.sh
```
