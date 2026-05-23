---
Status: Landed
Date: 2026-05-23
Scope: C mimalloc execution diagnostic owner-local counter exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-115-HAKO-ALLOC-USIZE-C-MIMALLOC-EXECUTION-DIAGNOSTIC-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_execution_diagnostic_box.hako
  - apps/hako-alloc-allocator-comparison-c-mimalloc-execution-diagnostics-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_execution_diagnostics_guard.sh
---

# 294x-116 Hako Alloc Usize C Mimalloc Execution Diagnostic Counters

## Decision

Migrate only the selected
`HakoAllocAllocatorComparisonCMimallocExecutionDiagnostic` owner-local
monotonic counters to exact `usize` storage:

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

The MIMAP-449A C mimalloc execution diagnostics guard now asserts these fields
are exact `usize` in the typed-object plan.

## Stop Line

This row does not migrate:

- `run_count`, because it is external execution payload evidence;
- `last_reason`, because it is reason vocabulary;
- report fields and `ReportFields` mirrors, because they remain signed
  comparison payload/mirror seams until their own row;
- C mimalloc execution behavior, process allocator replacement, hooks, backend
  matcher additions, provider package generation, worker/TLS, threads, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_execution_diagnostics_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
