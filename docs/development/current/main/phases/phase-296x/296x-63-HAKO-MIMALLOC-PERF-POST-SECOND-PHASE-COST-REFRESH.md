---
Status: Current
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

## Stop Line

Do not optimize in this row. Do not claim parity, activate providers, replace
the process allocator, install hooks, or select hakozuna.
