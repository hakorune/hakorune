---
Status: Current
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

## Stop Line

Do not optimize in this row. Do not claim parity, activate providers, replace
the process allocator, install hooks, or select hakozuna.
