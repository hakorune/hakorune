---
Status: Landed
Date: 2026-05-23
Scope: select the next owner-local production exact `usize` field group after the C mimalloc execution inventory counter migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-114-HAKO-ALLOC-USIZE-C-MIMALLOC-EXECUTION-INVENTORY-COUNTERS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_execution_diagnostic_box.hako
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_execution_diagnostics_guard.sh
---

# 294x-115 Hako Alloc Usize C Mimalloc Execution Diagnostic Counter Selection

## Decision

Select the owner-local monotonic counters in
`HakoAllocAllocatorComparisonCMimallocExecutionDiagnostic` as
`HAKO-ALLOC-USIZE-FIELD-GROUP-137`:

- `diagnostic_count`
- `ready_count`
- `blocked_count`
- `missing_runner_blocked_count`
- `missing_workload_blocked_count`
- `missing_hako_metrics_blocked_count`
- `missing_output_contract_blocked_count`
- `missing_memory_usage_contract_blocked_count`
- `missing_evidence_storage_blocked_count`
- `missing_run_count_blocked_count`
- `invalid_run_count_blocked_count`

These fields count the MIMAP-449A C mimalloc execution diagnostic owner's local
classifications and blocked outcomes. They do not carry run-count payloads,
readiness flags, reason vocabulary, or provider / host allocator state.

## Stop Line

This selection does not migrate:

- `run_count`, because it is external execution payload evidence;
- `last_reason`, because it is reason vocabulary;
- `HakoAllocAllocatorComparisonCMimallocExecutionDiagnosticReportFields` and
  `HakoAllocAllocatorComparisonCMimallocExecutionDiagnosticReport` fields,
  because report mirrors stay signed until their own row;
- C mimalloc execution behavior, process allocator replacement, hooks, backend
  matcher additions, provider package generation, worker/TLS, threads, or
  `#[global_allocator]`.

## Verification

Selection-only row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
