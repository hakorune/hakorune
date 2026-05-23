---
Status: Landed
Date: 2026-05-23
Scope: C mimalloc result reporting inventory owner-local counter exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-121-HAKO-ALLOC-USIZE-C-MIMALLOC-RESULT-REPORTING-INVENTORY-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_reporting_inventory_box.hako
  - apps/hako-alloc-allocator-comparison-c-mimalloc-result-reporting-inventory-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_reporting_inventory_guard.sh
---

# 294x-122 Hako Alloc Usize C Mimalloc Result Reporting Inventory Counters

## Decision

Migrate only the selected
`HakoAllocAllocatorComparisonCMimallocResultReportingInventory` owner-local
monotonic counters to exact `usize` storage:

- `reporting_count`
- `ready_count`
- `blocked_count`
- `missing_summary_diagnostic_reject_count`
- `blocked_summary_diagnostic_reject_count`

The MIMAP-460A C mimalloc result reporting inventory guard now asserts these
fields are exact `usize` in the typed-object plan.

## Stop Line

This row does not migrate:

- comparison payloads such as allocation counts, requested bytes, RSS bytes, or
  deltas;
- `last_reason`, because it is reason vocabulary;
- report fields and `ReportFields` mirrors, because they remain signed
  comparison payload/mirror seams until their own row;
- performance conclusions, memory conclusions, repeated benchmark execution,
  process allocator replacement, hooks, backend matcher additions, provider
  package generation, worker/TLS, threads, or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_reporting_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
