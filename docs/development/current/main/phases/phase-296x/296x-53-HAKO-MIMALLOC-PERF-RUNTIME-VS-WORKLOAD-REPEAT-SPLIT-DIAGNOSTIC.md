---
Status: Current
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

Run the same repeat ladder for `representative-empty-v0` and compare it to the
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

If empty scaling explains most of the small-block gap growth, continue runtime
baseline diagnostics. If small-block grows beyond empty scaling, then split
compiler lowering from allocator algorithm in a later row.

## Stop Line

Do not optimize, claim parity, activate providers, replace the process
allocator, install hooks, or select hakozuna in this row.
