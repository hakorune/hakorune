---
Status: Landed
Date: 2026-05-27
Scope: refresh in-process gap taxonomy after the first keeper optimization.
Blocker: HAKO-MIMALLOC-PERF-POST-KEEPER-TAXONOMY-REFRESH-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-58-HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION.md
---

# 296x-59 Hako Mimalloc Post Keeper Taxonomy Refresh

## Purpose

Re-run in-process gap taxonomy after the first keeper optimization before
choosing another optimization or returning to port feature work.

## Required Input

```text
optimization_kind=page_model_reuse_via_reset_to_fresh
output_contract=hako-mimalloc-in-process-operation-repeat-measurement-v0
winner_claim=0
```

## Required Output

```text
output_contract=hako-mimalloc-post-keeper-taxonomy-refresh-v0
previous_hako_external_elapsed_median_ms=330
current_hako_external_elapsed_median_ms=280
improvement_ms=50
remaining_gap_ms=276
next_diagnostic
next_optimization_allowed=0|1
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Evidence

```text
output_contract=hako-mimalloc-post-keeper-taxonomy-refresh-v0
optimization_kind=page_model_reuse_via_reset_to_fresh
measurement_profile=hako-mimalloc-in-process-operation-repeat-v0
timing_repeat_kind=in-process-operation-loop-v0
operation_repeat=8192
process_repeat=3
hako_compile_build_excluded=1
c_compile_build_excluded=1
external_timing_collector_hako=usr_bin_time_elapsed
external_timing_collector_c=python_perf_counter_subprocess
external_timing_collectors_same=0
hako_body_timing_available=0
c_body_timing_available=1
body_elapsed_comparable=0
body_elapsed_primary=0
hako_work_shape=page_model_acquire_release_reset
c_work_shape=mi_malloc_memset_mi_free
payload_write_equivalent=0
allocator_backend_equivalent=0
operation_count_equivalent=1
requested_bytes_equivalent=1
release_order_equivalent=1
same_workload_semantics=partial
interpretation_scope=operation-count-parity-only
previous_hako_external_elapsed_median_ms=330
current_hako_external_elapsed_median_ms=280
current_c_external_elapsed_median_ms=4
improvement_ms=50
remaining_gap_ms=276
gap_owner=allocator_algorithm
gap_confidence=high
next_diagnostic=phase_cost_ablation_reset_alloc_release
next_optimization_allowed=0
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Guard

```text
tools/checks/k2_wide_phase296x_hako_mimalloc_perf_post_keeper_taxonomy_refresh_guard.sh
```

## Stop Line

Do not claim parity, activate providers, replace the process allocator, install
hooks, select hakozuna, or batch multiple new optimizations in this row.
