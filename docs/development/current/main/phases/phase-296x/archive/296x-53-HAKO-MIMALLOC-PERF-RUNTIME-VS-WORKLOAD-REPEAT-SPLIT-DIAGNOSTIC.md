---
Status: Landed
Date: 2026-05-27
Scope: split process-invocation scaling gap between empty runtime baseline and workload body cost.
Blocker: HAKO-MIMALLOC-PERF-RUNTIME-VS-WORKLOAD-REPEAT-SPLIT-DIAGNOSTIC-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-52-HAKO-MIMALLOC-PERF-RUNTIME-BASELINE-SCALING-DIAGNOSTIC.md
---

# 296x-53 Hako Mimalloc Runtime vs Workload Repeat Split Diagnostic

## Purpose

Row 52 showed that the `.hako`/C median gap grows with process invocation
repeat. Since the current `operation_repeat` repeats the EXE process, this row
must compare empty-workload scaling against small-block scaling before blaming
compiler lowering or allocator algorithm cost.

## Required Input

```text
output_contract=hako-mimalloc-runtime-baseline-scaling-diagnostic-v0
per_invocation_growth_observed=1
refreshed_gap_owner=process_invocation_scaling_gap
refreshed_gap_confidence=medium
next_diagnostic=runtime_vs_workload_repeat_split_diagnostic
next_optimization_allowed=0
winner_claim=0
```

## Required Diagnostic

Ran the same repeat ladder for `representative-empty-v0` and compared it to the
small-block ladder:

```text
empty_workload_id=representative-empty-v0
small_workload_id=representative-small-block-v0
operation_repeat=128|1024|8192
sample_count=3
selected_gap_owner=hako_runtime_baseline|compiler_lowering|allocator_algorithm
selected_gap_confidence=low|medium|high
next_optimization_allowed=0|1
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
```

## Evidence

```text
output_contract=hako-mimalloc-runtime-vs-workload-repeat-split-v0
empty_workload_id=representative-empty-v0
small_workload_id=representative-small-block-v0
repeat_0_operation_repeat=128
repeat_0_empty_elapsed_gap_ms=10
repeat_0_small_elapsed_gap_ms=10
repeat_1_operation_repeat=1024
repeat_1_empty_elapsed_gap_ms=110
repeat_1_small_elapsed_gap_ms=120
repeat_2_operation_repeat=8192
repeat_2_empty_elapsed_gap_ms=820
repeat_2_small_elapsed_gap_ms=770
empty_gap_growth_ms=810
small_gap_growth_ms=760
runtime_explains_ratio_pct=106
selected_gap_owner=benchmark_harness
selected_gap_confidence=high
next_diagnostic=in_process_operation_repeat_contract
next_optimization_allowed=0
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Empty-workload process-repeat growth explains the small-block process-repeat
growth, so the next row must define an in-process operation-repeat measurement
contract before compiler/allocator optimization.

## Guard

```text
tools/checks/k2_wide_phase296x_hako_mimalloc_perf_runtime_vs_workload_repeat_split_guard.sh
```

## Stop Line

Do not optimize, claim parity, activate providers, replace the process
allocator, install hooks, or select hakozuna in this row.
