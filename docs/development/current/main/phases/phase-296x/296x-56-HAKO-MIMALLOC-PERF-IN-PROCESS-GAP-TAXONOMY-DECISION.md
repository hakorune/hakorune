---
Status: Landed
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
gap_owner=allocator_algorithm
gap_confidence=low
next_diagnostic=compiler_allocator_owner_split_diagnostic
next_optimization_allowed=0
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Evidence

```text
output_contract=hako-mimalloc-in-process-gap-taxonomy-decision-v0
input_contract=hako-mimalloc-in-process-operation-repeat-measurement-v0
workload_id=representative-small-block-v0
operation_repeat=8192
process_repeat=3
hako_external_elapsed_median_ms=330
c_external_elapsed_median_ms=4
c_body_elapsed_median_ns=3240447
external_elapsed_median_gap_ms=326
gap_owner=allocator_algorithm
gap_confidence=low
next_diagnostic=compiler_allocator_owner_split_diagnostic
next_optimization_allowed=0
optimization_started=0
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

The process-repeat harness has been excluded. The remaining gap is in the hako
workload path, but this row does not yet separate compiler lowering cost from
allocator model/algorithm overhead, so optimization stays closed.

## Guard

```text
tools/checks/k2_wide_phase296x_hako_mimalloc_perf_in_process_gap_taxonomy_decision_guard.sh
```

## Stop Line

Do not optimize, claim parity, activate providers, replace the process
allocator, install hooks, or select hakozuna in this row.
