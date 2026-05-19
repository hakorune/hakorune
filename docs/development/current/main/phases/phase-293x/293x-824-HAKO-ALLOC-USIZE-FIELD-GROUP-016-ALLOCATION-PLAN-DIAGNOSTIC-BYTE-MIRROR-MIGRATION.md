# 293x-824 HAKO-ALLOC-USIZE-FIELD-GROUP-016 Allocation-Plan Diagnostic Byte Mirror Migration

Status: selected current
Date: 2026-05-19

## Decision

Migrate the allocation-plan diagnostic mirror byte fields to exact `usize`.

This row follows `HAKO-ALLOC-USIZE-FIELD-GROUP-015`, which closed out the
allocation-plan report byte/capacity group. The diagnostic mirror fields are
observer-only copies of already-migrated allocation-plan byte facts, so they can
move as a separate narrow field group without changing allocation behavior.

## Scope

Change only these fields on
`HakoAllocSegmentArenaBackingModeledAllocationPlanDiagnosticReport` and its
local `HakoAllocSegmentArenaBackingModeledAllocationPlanDiagnosticReportFields`
record carrier:

```text
last_report_planned_backing_bytes
last_report_planned_committed_bytes
last_report_remaining_source_bytes
```

These fields mirror non-negative allocation-plan byte facts. They do not carry
sentinels and do not own counters, reasons, tokens, ids, or status
vocabularies.

## Stop Lines

- No second field-group migration in this row.
- No broad `i64` to `usize` rewrite.
- No allocation-plan report migration.
- No allocation-apply, allocation-ledger, or release-candidate migration.
- No migration of diagnostic counters.
- No migration of reason/status/flag vocabularies.
- No migration of token/id/generation fields.
- No migration of any sentinel-bearing field.
- No new backend route or `.inc` owner-name matcher.
- No runtime sum materialization.
- No cross-function `Result` direct ABI.
- No real raw pointer residence, arena backing execution, segment-map mutation,
  atomic bitmap execution, OSVM/page-source execution, provider activation,
  hooks, host allocator replacement, or `#[global_allocator]`.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_plan_diagnostics_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_plan_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

After this migration, close out the allocation-plan diagnostic mirror byte field
group before selecting another allocator byte/capacity group.
