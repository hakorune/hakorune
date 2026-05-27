---
Status: Landed
Date: 2026-05-27
Scope: apply one acquire-side allocator-model optimization after phase-cost ablation.
Blocker: HAKO-MIMALLOC-PERF-SECOND-KEEPER-OPTIMIZATION-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-60-HAKO-MIMALLOC-PERF-PHASE-COST-ABLATION.md
---

# 296x-61 Hako Mimalloc Second Keeper Optimization

## Purpose

Apply exactly one `.hako` allocator-model optimization to the dominant alloc
phase selected by row 60.

## Required Input

```text
output_contract=hako-mimalloc-phase-cost-ablation-v0
hako_level_vs_mirbuilder_level=hako_allocator_model_primary
dominant_phase=alloc
next_optimization_target=acquire_usize_fast_path_and_invariant_hoist
next_optimization_allowed=1
winner_claim=0
```

## Required Output

```text
optimization_kind
target_phase=alloc
before_full_elapsed_median_ms=280
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
optimization_kind=acquire_usize_free_top_fast_path
target_phase=alloc
before_reset_alloc_only_elapsed_median_ms=190
after_reset_alloc_only_elapsed_median_ms=170
before_full_elapsed_median_ms=280
after_full_elapsed_median_ms=260
improvement_ms=20
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
tools/checks/k2_wide_phase296x_hako_mimalloc_perf_second_keeper_optimization_guard.sh
```

## Stop Line

Keep this row to one acquire-side optimization. Do not mix reset, release,
provider, replacement, hook, global allocator, or port-feature work into this
row.
