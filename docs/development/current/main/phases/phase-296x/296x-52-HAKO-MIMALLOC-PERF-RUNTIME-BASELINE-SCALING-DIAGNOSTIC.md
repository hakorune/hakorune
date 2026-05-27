---
Status: Landed
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

This row must determine whether the gap stays fixed as process invocation
repeat increases, or whether the repeated exact-EXE execution gap grows.

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

Ran a small repeat ladder for the same workload family, keeping build/compile
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

## Evidence

```text
output_contract=hako-mimalloc-runtime-baseline-scaling-diagnostic-v0
workload_id=representative-small-block-v0
sample_count=3
repeat_0_operation_repeat=128
repeat_0_elapsed_gap_ms=10
repeat_1_operation_repeat=1024
repeat_1_elapsed_gap_ms=90
repeat_2_operation_repeat=8192
repeat_2_elapsed_gap_ms=750
gap_growth_ms=740
per_invocation_gap_growth_us=91
per_invocation_growth_observed=1
runtime_baseline_fixed_gap_observed=0
refreshed_gap_owner=process_invocation_scaling_gap
refreshed_gap_confidence=medium
next_diagnostic=runtime_vs_workload_repeat_split_diagnostic
next_optimization_allowed=0
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

The 10ms low-repeat gap is not just fixed process/runtime baseline. At 8192
process invocations the median gap grows to 750ms. Because this ladder repeats
process execution, the next row must compare empty-workload and small-block
repeat scaling before compiler or allocator optimization starts.

## Guard

```text
tools/checks/k2_wide_phase296x_hako_mimalloc_perf_runtime_baseline_scaling_diagnostic_guard.sh
```

## Stop Line

Do not optimize, claim parity, activate providers, replace the process
allocator, install hooks, or select hakozuna in this row.
