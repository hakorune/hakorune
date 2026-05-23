---
Status: Landed
Date: 2026-05-23
Scope: select the next owner-local production exact `usize` field group after the C mimalloc result ledger diagnostic counter migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-112-HAKO-ALLOC-USIZE-C-MIMALLOC-RESULT-LEDGER-DIAGNOSTIC-COUNTERS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_execution_inventory_box.hako
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_execution_inventory_guard.sh
---

# 294x-113 Hako Alloc Usize C Mimalloc Execution Inventory Counter Selection

## Decision

Select the owner-local monotonic counters in
`HakoAllocAllocatorComparisonCMimallocExecutionInventory` as
`HAKO-ALLOC-USIZE-FIELD-GROUP-135`:

- `inventory_count`
- `accepted_count`
- `reject_count`
- `missing_runner_reject_count`
- `missing_workload_reject_count`
- `missing_hako_metrics_reject_count`
- `missing_output_contract_reject_count`
- `missing_memory_usage_contract_reject_count`
- `missing_evidence_storage_reject_count`
- `missing_run_count_reject_count`
- `invalid_run_count_reject_count`

These fields count the MIMAP-448A C mimalloc execution inventory owner's local
attempts and reject outcomes. They do not carry run-count payloads, readiness
flags, reason vocabulary, or provider / host allocator state.

## Stop Line

This selection does not migrate:

- `run_count`, because it is external execution payload evidence;
- `last_reason`, because it is reason vocabulary;
- `HakoAllocAllocatorComparisonCMimallocExecutionInventoryReportFields` and
  `HakoAllocAllocatorComparisonCMimallocExecutionInventoryReport` fields,
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
