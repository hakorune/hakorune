---
Status: Landed
Date: 2026-05-23
Scope: explicit C mimalloc runner execution pilot owner-local counter exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-105-HAKO-ALLOC-USIZE-C-MIMALLOC-EXPLICIT-RUNNER-EXECUTION-PILOT-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_box.hako
  - apps/hako-alloc-allocator-comparison-c-mimalloc-explicit-runner-execution-pilot-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_guard.sh
---

# 294x-106 Hako Alloc Usize C Mimalloc Explicit Runner Execution Pilot Counters

## Decision

Migrate only the selected
`HakoAllocAllocatorComparisonCMimallocExplicitRunnerExecutionPilot`
owner-local monotonic counters to exact `usize` storage:

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

The MIMAP-451A explicit C mimalloc runner execution pilot guard now asserts
these fields are exact `usize` in the typed-object plan.

## Stop Line

This row does not migrate:

- `last_reason`, because it is reason vocabulary;
- runner evidence records, memory/RSS payloads, result codes, or stable output
  contract flags;
- report fields and `ReportFields` mirrors, because they remain signed
  comparison payload/mirror seams until their own row;
- explicit C runner execution behavior, process allocator replacement, hooks,
  backend matcher additions, provider package generation, worker/TLS,
  threads, or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
