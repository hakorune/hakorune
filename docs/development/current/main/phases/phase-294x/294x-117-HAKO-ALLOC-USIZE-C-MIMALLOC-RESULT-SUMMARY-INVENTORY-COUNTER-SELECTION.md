---
Status: Landed
Date: 2026-05-23
Scope: select the next owner-local production exact `usize` field group after the C mimalloc execution diagnostic counter migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-116-HAKO-ALLOC-USIZE-C-MIMALLOC-EXECUTION-DIAGNOSTIC-COUNTERS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_summary_inventory_box.hako
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_summary_inventory_guard.sh
---

# 294x-117 Hako Alloc Usize C Mimalloc Result Summary Inventory Counter Selection

## Decision

Select the owner-local monotonic counters in
`HakoAllocAllocatorComparisonCMimallocResultSummaryInventory` as
`HAKO-ALLOC-USIZE-FIELD-GROUP-139`:

- `summary_count`
- `ready_count`
- `blocked_count`
- `missing_ledger_reject_count`
- `missing_diagnostic_reject_count`
- `blocked_diagnostic_reject_count`

These fields count the MIMAP-457A C mimalloc result summary inventory owner's
local classifications and reject outcomes. They do not carry allocation-count
payloads, requested-byte payloads, RSS evidence, deltas, readiness flags, reason
vocabulary, or provider / host allocator state.

## Stop Line

This selection does not migrate:

- `hako_allocation_count`, `hako_release_count`, `hako_reject_count`,
  `hako_requested_bytes`, `c_allocation_count`, `c_free_count`,
  `c_requested_bytes`, `c_peak_rss_bytes`, `allocation_count_delta`, or
  `requested_bytes_delta`, because they are comparison/evidence payloads;
- `last_reason`, because it is reason vocabulary;
- `HakoAllocAllocatorComparisonCMimallocResultSummaryInventoryReportFields` and
  `HakoAllocAllocatorComparisonCMimallocResultSummaryInventoryReport` fields,
  because report mirrors stay signed until their own row;
- performance conclusions, memory conclusions, repeated benchmark execution,
  process allocator replacement, hooks, backend matcher additions, provider
  package generation, worker/TLS, threads, or `#[global_allocator]`.

## Verification

Selection-only row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
