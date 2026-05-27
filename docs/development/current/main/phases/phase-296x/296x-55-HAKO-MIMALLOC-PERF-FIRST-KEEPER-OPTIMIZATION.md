---
Status: Planned
Date: 2026-05-27
Scope: apply the first evidence-backed optimization only if in-process diagnostics allow it.
Blocker: HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
---

# 296x-55 Hako Mimalloc First Keeper Optimization

## Purpose

Apply exactly one optimization only when owner diagnostics select this row.

## Required Input

```text
decision=enter_first_keeper_optimization
selected_next_row=HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION-296X-001
next_optimization_allowed=1
gap_owner=compiler_lowering|allocator_algorithm
winner_claim=0
```

## Stop Line

Do not start from noisy, low-confidence, benchmark-harness, provider-wrapper, or
hako-runtime-baseline evidence. Do not combine compiler and allocator changes in
one row.
