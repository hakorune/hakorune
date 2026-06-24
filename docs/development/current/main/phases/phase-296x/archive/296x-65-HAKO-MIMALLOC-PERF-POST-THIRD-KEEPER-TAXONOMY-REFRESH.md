---
Status: Landed
Date: 2026-05-27
Scope: refresh in-process taxonomy after the third keeper optimization.
Blocker: HAKO-MIMALLOC-PERF-POST-THIRD-KEEPER-TAXONOMY-REFRESH-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-64-HAKO-MIMALLOC-PERF-THIRD-KEEPER-OPTIMIZATION.md
---

# 296x-65 Hako Mimalloc Post Third Keeper Taxonomy Refresh

## Purpose

Refresh the in-process gap after the known-active small-cycle keeper before
choosing another optimization or switching to feature-port inventory.

## Required Input

```text
optimization_kind=known_active_small_cycle_fast_path
target_phase=known_active_small_cycle
before_full_elapsed_median_ms=250
after_full_elapsed_median_ms=240
winner_claim=0
```

## Required Output

```text
output_contract=hako-mimalloc-post-third-keeper-taxonomy-refresh-v0
current_hako_external_elapsed_median_ms=240
current_c_external_elapsed_median_ms
remaining_gap_ms
gap_owner
gap_confidence
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
output_contract=hako-mimalloc-post-third-keeper-taxonomy-refresh-v0
optimization_kind=known_active_small_cycle_fast_path
measurement_profile=hako-mimalloc-in-process-operation-repeat-v0
timing_repeat_kind=in-process-operation-loop-v0
operation_repeat=8192
process_repeat=3
hako_compile_build_excluded=1
c_compile_build_excluded=1
external_timing_collectors_same=0
body_elapsed_comparable=0
body_elapsed_primary=0
hako_work_shape=page_model_known_active_small_cycle
c_work_shape=mi_malloc_memset_mi_free
same_workload_semantics=partial
interpretation_scope=operation-count-parity-only
current_hako_external_elapsed_median_ms=240
current_c_external_elapsed_median_ms=4
remaining_gap_ms=236
gap_owner=allocator_algorithm
gap_confidence=high
optimization_checkpoint=small_model_fast_path_plateau
next_diagnostic=port_feature_gap_inventory
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
tools/checks/k2_wide_phase296x_hako_mimalloc_perf_post_third_keeper_taxonomy_refresh_guard.sh
```

## Stop Line

Do not optimize in this row. Do not claim parity, activate providers, replace
the process allocator, install hooks, or select hakozuna.
