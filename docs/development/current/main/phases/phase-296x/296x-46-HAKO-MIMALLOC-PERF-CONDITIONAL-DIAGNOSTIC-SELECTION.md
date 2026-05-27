---
Status: Current
Date: 2026-05-27
Scope: choose the next narrow diagnostic from row 45 gap taxonomy evidence.
Blocker: HAKO-MIMALLOC-PERF-CONDITIONAL-DIAGNOSTIC-SELECTION-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-45-HAKO-MIMALLOC-PERF-GAP-TAXONOMY-ADAPTER.md
---

# 296x-46 Hako Mimalloc Conditional Diagnostic Selection

## Purpose

Choose the next diagnostic from row 45 output without automatically widening
the benchmark contract.

## Input

Use:

```text
output_contract=hako-mimalloc-gap-taxonomy-v0
gap_owner=<one primary owner>
evidence_quality=stable|noisy
gap_confidence=low|medium|high
next_diagnostic
next_optimization_allowed=0|1
winner_claim=0
```

## Selection Rules

```text
if gap_owner=benchmark_harness or evidence_quality=noisy:
  select measurement_hygiene_refresh

if gap_owner=hako_runtime_baseline:
  select empty_workload_or_repeat_scaling_runtime_diagnostic

if gap_owner=compiler_lowering:
  select mir_or_body_shape_diagnostic

if gap_owner=allocator_algorithm:
  select operation_repeat_scaling_or_allocator_counter_diagnostic

if gap_owner=c_abi_memory_bridge:
  select c_runner_api_or_load_boundary_diagnostic

if gap_owner=provider_wrapper:
  select provider_explicit_call_overhead_diagnostic
```

## Stop Line

Do not optimize, claim parity, activate providers, replace the process
allocator, install hooks, or make `body_elapsed_ns` primary in this row.
