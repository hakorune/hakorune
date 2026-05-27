---
Status: Current
Date: 2026-05-27
Scope: classify the in-process hako/C gap before the first optimization.
Blocker: HAKO-MIMALLOC-PERF-IN-PROCESS-GAP-TAXONOMY-DECISION-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-55-HAKO-MIMALLOC-PERF-IN-PROCESS-OPERATION-REPEAT-PILOT.md
---

# 296x-56 Hako Mimalloc In-Process Gap Taxonomy Decision

## Purpose

Consume the first in-process operation-repeat measurement and classify the
remaining gap before any keeper optimization starts.

## Required Input

```text
output_contract=hako-mimalloc-in-process-operation-repeat-measurement-v0
timing_repeat_kind=in-process-operation-loop-v0
process_invocation_repeat=0
operation_repeat=8192
same_workload=1
same_operation_count=1
winner_claim=0
```

## Required Decision

```text
output_contract=hako-mimalloc-in-process-gap-taxonomy-decision-v0
gap_owner=compiler_lowering|allocator_algorithm|hako_runtime_baseline|benchmark_harness
gap_confidence=low|medium|high
next_diagnostic
next_optimization_allowed=0|1
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Only `compiler_lowering` or `allocator_algorithm` with medium-or-better
confidence may open the first keeper optimization row.

## Stop Line

Do not optimize, claim parity, activate providers, replace the process
allocator, install hooks, or select hakozuna in this row.
