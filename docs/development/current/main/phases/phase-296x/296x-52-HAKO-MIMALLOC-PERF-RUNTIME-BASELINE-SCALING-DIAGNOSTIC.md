---
Status: Current
Date: 2026-05-27
Scope: separate fixed runtime baseline cost from per-operation hako mimalloc cost.
Blocker: HAKO-MIMALLOC-PERF-RUNTIME-BASELINE-SCALING-DIAGNOSTIC-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-51-HAKO-MIMALLOC-PERF-OWNER-CONFIDENCE-REFRESH.md
---

# 296x-52 Hako Mimalloc Runtime Baseline Scaling Diagnostic

## Purpose

Row 51 raised the `hako_runtime_baseline` owner confidence to medium by showing
that an empty workload has the same 10ms median external elapsed gap as the
small-block workload.

This row must determine whether the gap stays fixed as `operation_repeat`
increases, or whether per-operation cost starts to dominate.

## Required Input

```text
output_contract=hako-mimalloc-owner-confidence-refresh-v0
refreshed_gap_owner=hako_runtime_baseline
refreshed_gap_confidence=medium
next_diagnostic=repeat_scaling_runtime_diagnostic
next_optimization_allowed=0
winner_claim=0
```

## Required Diagnostic

Run a small repeat ladder for the same workload family, keeping build/compile
outside the measured sample and keeping `external_elapsed_ms` primary:

```text
workload_id=representative-small-block-v0
operation_repeat=128|1024|8192
sample_count=3
warmup_count=1
body_elapsed_ns_secondary=1 if available
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
```

If the median gap remains roughly fixed, the next row should treat it as
runtime/process baseline rather than allocator algorithm cost. If the gap grows
with repeat count, the next row may return to allocator or compiler owner
diagnostics.

## Stop Line

Do not optimize, claim parity, activate providers, replace the process
allocator, install hooks, or select hakozuna in this row.
