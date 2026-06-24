---
Status: Landed
Date: 2026-05-27
Scope: refresh phase-cost ablation after the second keeper optimization.
Blocker: HAKO-MIMALLOC-PERF-POST-SECOND-PHASE-COST-REFRESH-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-62-HAKO-MIMALLOC-PERF-POST-SECOND-KEEPER-TAXONOMY-REFRESH.md
---

# 296x-63 Hako Mimalloc Post Second Phase Cost Refresh

## Purpose

Refresh reset / alloc / release phase costs after the acquire fast path before
choosing a third keeper optimization.

## Required Input

```text
output_contract=hako-mimalloc-post-second-keeper-taxonomy-refresh-v0
gap_owner=allocator_algorithm
gap_confidence=high
next_diagnostic=post_second_phase_cost_ablation_refresh
next_optimization_allowed=0
winner_claim=0
```

## Required Output

```text
output_contract=hako-mimalloc-phase-cost-ablation-v0
reset_only_elapsed_median_ms
reset_alloc_only_elapsed_median_ms
full_elapsed_median_ms=260
alloc_only_estimated_ms
release_only_elapsed_median_ms
dominant_phase=reset|alloc|release|mixed
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
output_contract=hako-mimalloc-phase-cost-ablation-v0
measurement_profile=hako-mimalloc-phase-cost-ablation-v0
timing_repeat_kind=in-process-operation-loop-v0
operation_repeat=8192
process_repeat=3
runtime_config_profile=empty
external_timing_collector_hako=usr_bin_time_elapsed
hako_body_timing_available=0
body_elapsed_primary=0
phase_cost_method=median_difference_ablation
release_only_estimated=1
hako_level_vs_mirbuilder_level=hako_allocator_model_primary
mirbuilder_owner=secondary_later
work_shape=page_model_reset_acquire_release
reset_only_elapsed_median_ms=60
reset_alloc_only_elapsed_median_ms=170
full_elapsed_median_ms=250
alloc_only_estimated_ms=110
alloc_release_elapsed_median_ms=190
release_only_elapsed_median_ms=80
dominant_phase=alloc
next_optimization_target=known_active_small_cycle_fast_path
next_optimization_allowed=1
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Guard

```text
tools/checks/k2_wide_phase296x_hako_mimalloc_perf_post_second_phase_cost_refresh_guard.sh
```

## Stop Line

Do not optimize in this row. Do not claim parity, activate providers, replace
the process allocator, install hooks, or select hakozuna.
