---
Status: Landed
Date: 2026-05-23
Scope: select the next owner-local production exact `usize` field group after the C mimalloc result summary diagnostic counter migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-120-HAKO-ALLOC-USIZE-C-MIMALLOC-RESULT-SUMMARY-DIAGNOSTIC-COUNTERS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_reporting_inventory_box.hako
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_reporting_inventory_guard.sh
---

# 294x-121 Hako Alloc Usize C Mimalloc Result Reporting Inventory Counter Selection

## Decision

Select the owner-local monotonic counters in
`HakoAllocAllocatorComparisonCMimallocResultReportingInventory` as
`HAKO-ALLOC-USIZE-FIELD-GROUP-143`:

- `reporting_count`
- `ready_count`
- `blocked_count`
- `missing_summary_diagnostic_reject_count`
- `blocked_summary_diagnostic_reject_count`

These fields count the MIMAP-460A C mimalloc result reporting inventory owner's
local classifications and reject outcomes. They do not carry allocation-count
payloads, requested-byte payloads, RSS evidence, deltas, readiness flags, reason
vocabulary, conclusions, or provider / host allocator state.

## Stop Line

This selection does not migrate:

- comparison payloads such as allocation counts, requested bytes, RSS bytes, or
  deltas;
- `last_reason`, because it is reason vocabulary;
- `HakoAllocAllocatorComparisonCMimallocResultReportingInventoryReportFields`
  and `HakoAllocAllocatorComparisonCMimallocResultReportingInventoryReport`
  fields, because report mirrors stay signed until their own row;
- performance conclusions, memory conclusions, repeated benchmark execution,
  process allocator replacement, hooks, backend matcher additions, provider
  package generation, worker/TLS, threads, or `#[global_allocator]`.

## Verification

Selection-only row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
