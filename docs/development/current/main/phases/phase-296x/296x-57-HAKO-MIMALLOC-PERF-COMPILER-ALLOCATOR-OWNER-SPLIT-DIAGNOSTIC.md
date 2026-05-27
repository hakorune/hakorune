---
Status: Landed
Date: 2026-05-27
Scope: split the in-process hako workload gap between compiler lowering and allocator algorithm owners.
Blocker: HAKO-MIMALLOC-PERF-COMPILER-ALLOCATOR-OWNER-SPLIT-DIAGNOSTIC-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-56-HAKO-MIMALLOC-PERF-IN-PROCESS-GAP-TAXONOMY-DECISION.md
---

# 296x-57 Hako Mimalloc Compiler/Allocator Owner Split Diagnostic

## Purpose

Split the first in-process hako workload gap between compiler lowering overhead
and allocator model/algorithm overhead before any optimization starts.

## Required Input

```text
output_contract=hako-mimalloc-in-process-gap-taxonomy-decision-v0
gap_owner=allocator_algorithm
gap_confidence=low
next_diagnostic=compiler_allocator_owner_split_diagnostic
next_optimization_allowed=0
winner_claim=0
```

## Required Diagnostic

Used narrow loop-shell evidence:

```text
mir_or_body_shape_evidence=0|1
allocator_counter_or_behavior_evidence=0|1
selected_gap_owner=compiler_lowering|allocator_algorithm
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
output_contract=hako-mimalloc-compiler-allocator-owner-split-v0
input_contract=hako-mimalloc-in-process-operation-repeat-measurement-v0
workload_id=representative-small-block-v0
shell_workload_id=representative-loop-shell-v0
operation_repeat=8192
hako_external_elapsed_median_ms=330
c_external_elapsed_median_ms=4
external_elapsed_median_gap_ms=326
shell_hako_external_elapsed_median_ms=1
shell_explains_hako_ratio_pct=0
mir_or_body_shape_evidence=1
allocator_counter_or_behavior_evidence=1
selected_gap_owner=allocator_algorithm
selected_gap_confidence=high
selected_next_row=HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION-296X-001
next_optimization_allowed=1
optimization_started=0
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

The 8192-iteration loop shell measures at 1ms median, so compiler loop shell
cost does not explain the 330ms hako small-block measurement. The next row may
open exactly one allocator-algorithm keeper optimization.

## Guard

```text
tools/checks/k2_wide_phase296x_hako_mimalloc_perf_compiler_allocator_owner_split_guard.sh
```

## Stop Line

Do not optimize, claim parity, activate providers, replace the process
allocator, install hooks, or select hakozuna in this row.
