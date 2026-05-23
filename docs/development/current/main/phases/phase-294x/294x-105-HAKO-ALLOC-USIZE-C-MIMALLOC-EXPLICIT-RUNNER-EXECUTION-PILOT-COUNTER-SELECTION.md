---
Status: Landed
Date: 2026-05-23
Scope: select the next owner-local production exact `usize` field group after the abandoned/reclaim inventory counter migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-104-HAKO-ALLOC-USIZE-ABANDONED-RECLAIM-INVENTORY-COUNTERS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_box.hako
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_guard.sh
---

# 294x-105 Hako Alloc Usize C Mimalloc Explicit Runner Execution Pilot Counter Selection

## Decision

Select the owner-local monotonic counters in
`HakoAllocAllocatorComparisonCMimallocExplicitRunnerExecutionPilot` as
`HAKO-ALLOC-USIZE-FIELD-GROUP-127`:

- `pilot_count`
- `accepted_count`
- `reject_count`
- `missing_diagnostic_reject_count`
- `rejected_diagnostic_reject_count`
- `missing_runner_reject_count`
- `missing_output_reject_count`
- `missing_memory_evidence_reject_count`
- `missing_output_contract_reject_count`
- `failed_runner_reject_count`
- `invalid_run_count_reject_count`

These fields count the MIMAP-451A explicit C mimalloc runner execution pilot
owner's local attempts and reject outcomes. They do not carry runner payloads,
RSS evidence, result codes, reason vocabulary, stop-line flags, or provider /
host allocator state.

## Stop Line

This selection does not migrate:

- `last_reason`, because it is reason vocabulary;
- `HakoAllocAllocatorComparisonCMimallocExplicitRunnerRunEvidence` fields,
  because they are external runner payload evidence;
- `HakoAllocAllocatorComparisonCMimallocExplicitRunnerMemoryEvidence` fields,
  because requested bytes and RSS evidence remain comparison payload seams;
- `HakoAllocAllocatorComparisonCMimallocExplicitRunnerStopLineEvidence` fields,
  because they are closed-state flags;
- `HakoAllocAllocatorComparisonCMimallocExplicitRunnerExecutionPilotReportFields`
  and `HakoAllocAllocatorComparisonCMimallocExplicitRunnerExecutionPilotReport`
  fields, because report mirrors stay signed until their own row;
- explicit C runner execution behavior, process allocator replacement, hooks,
  backend matcher additions, provider package generation, worker/TLS,
  threads, or `#[global_allocator]`.

## Verification

Selection-only row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
