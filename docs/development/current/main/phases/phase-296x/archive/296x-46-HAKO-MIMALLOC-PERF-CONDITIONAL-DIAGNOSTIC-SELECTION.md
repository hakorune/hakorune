---
Status: Landed
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

## Evidence

Implemented:

```text
tools/allocator/hako_mimalloc_conditional_diagnostic_selector.py
```

The selector reads:

```text
output_contract=hako-mimalloc-gap-taxonomy-v0
```

and emits:

```text
output_contract=hako-mimalloc-conditional-diagnostic-selection-v0
selected_diagnostic=<one diagnostic>
measurement_hygiene_required=0|1
next_optimization_allowed=0|1
selected_next_row=HAKO-MIMALLOC-PERF-OWNER-NARROW-DIAGNOSTIC-296X-001
body_elapsed_ns_primary=0
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Selected Next

Select:

```text
HAKO-MIMALLOC-PERF-OWNER-NARROW-DIAGNOSTIC-296X-001
```

The next row should execute only the selected diagnostic. If the selected
diagnostic is `measurement_hygiene_refresh`, it may increase sample count and
prove build/compile exclusion; otherwise it should stay owner-specific.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_perf_conditional_diagnostic_selection_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
