---
Status: Landed
Date: 2026-05-27
Scope: apply one known-active small-cycle fast-path optimization after post-second phase-cost refresh.
Blocker: HAKO-MIMALLOC-PERF-THIRD-KEEPER-OPTIMIZATION-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-63-HAKO-MIMALLOC-PERF-POST-SECOND-PHASE-COST-REFRESH.md
---

# 296x-64 Hako Mimalloc Third Keeper Optimization

## Purpose

Apply exactly one `.hako` allocator-model optimization to the remaining
hot small-block cycle after the second keeper.

## Required Input

```text
output_contract=hako-mimalloc-phase-cost-ablation-v0
hako_level_vs_mirbuilder_level=hako_allocator_model_primary
dominant_phase=alloc
next_optimization_target=known_active_small_cycle_fast_path
next_optimization_allowed=1
winner_claim=0
```

## Required Output

```text
optimization_kind
target_phase=known_active_small_cycle
before_full_elapsed_median_ms=250
after_full_elapsed_median_ms
improvement_ms
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Evidence

```text
optimization_kind=known_active_small_cycle_fast_path
target_phase=known_active_small_cycle
before_reset_alloc_only_elapsed_median_ms=170
after_reset_alloc_only_elapsed_median_ms=160
before_full_elapsed_median_ms=250
after_full_elapsed_median_ms=240
improvement_ms=10
hako_level_vs_mirbuilder_level=hako_allocator_model_primary
mirbuilder_owner=secondary_later
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Guard

```text
tools/checks/k2_wide_phase296x_hako_mimalloc_perf_third_keeper_optimization_guard.sh
```

## Stop Line

Keep this row to one known-active small-cycle optimization. Do not mix reset,
provider, replacement, hook, global allocator, or feature-port work into this
row.
