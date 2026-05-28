---
Status: Landed
Date: 2026-05-28
Scope: measure exact-EXE timing after the field_get result-chain cleanup.
Blocker: POST-FIELD-GET-RESULT-CHAIN-CLEANUP-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-162-FIELD-GET-RESULT-CHAIN-CLEANUP-IMPLEMENTATION.md
  - tools/allocator/hako_mimalloc_post_field_get_cleanup_measurement.py
---

# 296x-163 Post Field Get Result Chain Cleanup Measurement

## Purpose

Measure the object-lifecycle exact-EXE workload after row162's structural
field_get result-chain cleanup. This row closes the keeper measurement without
opening winner, provider activation, replacement, hook, or global allocator
claims.

## Required Output

```text
output_contract=hako-mimalloc-post-field-get-result-chain-cleanup-measurement-v0
input_contract=field-get-result-chain-cleanup-implementation-v0
after_hako_elapsed_median_ms
previous_checkpoint_hako_elapsed_median_ms
measurement_effect
summary=ok
```

## Evidence

```text
output_contract=hako-mimalloc-post-field-get-result-chain-cleanup-measurement-v0
input_contract=field-get-result-chain-cleanup-implementation-v0
measurement_scope=object_lifecycle_facade_exact_exe_after_field_get_result_chain_cleanup
workload_id=representative-object-lifecycle-small-block-v0
operation_repeat=8192
sample_count=3
keeper=field_get_result_chain_cleanup
select_page_single_fast_path_count=524288
select_page_single_fallback_count=0
release_known_page_fast_path_count=524288
release_known_page_fallback_count=0
sample_0_hako_external_elapsed_ms=550
sample_1_hako_external_elapsed_ms=570
sample_2_hako_external_elapsed_ms=550
after_hako_elapsed_median_ms=550
after_hako_elapsed_min_ms=550
after_hako_elapsed_max_ms=570
previous_checkpoint_hako_elapsed_median_ms=560
delta_hako_elapsed_median_ms=-10
measurement_effect=improved
structural_keeper=1
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Interpretation:

```text
The row162 field_get cleanup is a keeper. It reduced MIR copy shape
structurally and measured slightly faster than the 560ms checkpoint. Treat the
timing gain as small but directionally positive; continue with owner-first
MIR observation rather than broad source rewrites.
```

## Next

```text
row164:
  post-field-get-cleanup-owner-refresh

Goal:
  refresh the current callsite/copy owner surface after row162 so the next
  optimization owner is selected from current evidence.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_post_field_get_result_chain_cleanup_measurement_guard.sh
```
