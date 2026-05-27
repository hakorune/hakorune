---
Status: Landed
Date: 2026-05-27
Scope: split remaining in-process allocator-model cost into reset, alloc, and release phases.
Blocker: HAKO-MIMALLOC-PERF-PHASE-COST-ABLATION-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-59-HAKO-MIMALLOC-PERF-POST-KEEPER-TAXONOMY-REFRESH.md
---

# 296x-60 Hako Mimalloc Phase Cost Ablation

## Purpose

Split the remaining 276ms in-process gap into `.hako` allocator model phases
before applying another optimization.

## Required Input

```text
output_contract=hako-mimalloc-post-keeper-taxonomy-refresh-v0
gap_owner=allocator_algorithm
gap_confidence=high
next_diagnostic=phase_cost_ablation_reset_alloc_release
next_optimization_allowed=0
winner_claim=0
```

## Required Output

```text
output_contract=hako-mimalloc-phase-cost-ablation-v0
reset_only_elapsed_median_ms
alloc_release_elapsed_median_ms
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
reset_alloc_only_elapsed_median_ms=190
full_elapsed_median_ms=280
alloc_only_estimated_ms=130
alloc_release_elapsed_median_ms=220
release_only_elapsed_median_ms=90
dominant_phase=alloc
next_optimization_target=acquire_usize_fast_path_and_invariant_hoist
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
tools/checks/k2_wide_phase296x_hako_mimalloc_perf_phase_cost_ablation_guard.sh
```

## Stop Line

Do not optimize in this row. Do not claim parity, activate providers, replace
the process allocator, install hooks, or select hakozuna.
