# 293x-820 HAKO-ALLOC-USIZE-FIELD-GROUP-012 Allocation-Apply Diagnostic Byte Mirror Migration

Status: landed
Date: 2026-05-19

## Decision

Migrate the allocation-apply diagnostic mirror byte fields to exact `usize`.

This row follows `HAKO-ALLOC-USIZE-FIELD-GROUP-011`, which closed out the
allocation-apply report byte/capacity group. The diagnostic mirror fields are
observer-only copies of already-migrated byte facts, so they can move as a
separate narrow field group without changing allocation behavior.

## Scope

Change only these fields on
`HakoAllocSegmentArenaBackingModeledAllocationApplyDiagnosticReport` and its
local `HakoAllocSegmentArenaBackingModeledAllocationApplyDiagnosticReportFields`
record carrier:

```text
last_report_applied_backing_bytes
last_report_applied_committed_bytes
last_report_remaining_source_bytes
```

These fields mirror non-negative allocation-apply byte facts. They do not carry
sentinels and do not own counters, reasons, tokens, ids, or status
vocabularies.

## Stop Lines

- No second field-group migration in this row.
- No broad `i64` to `usize` rewrite.
- No allocation-plan migration.
- No allocation-apply report migration.
- No allocation-ledger or release-candidate migration.
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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_apply_diagnostics_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_apply_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Notes

- Migrated only the allocation-apply diagnostic mirror byte fields on both the
  report box and its local ReportFields record carrier.
- Kept diagnostic counters, reasons, tokens, ids, status flags, and sentinels on
  `i64`.
- Strengthened the MIMAP-273A diagnostics guard to assert exact `usize`
  typed-object storage and record declaration types for the three mirror byte
  fields.
- Re-ran the MIMAP-274A closeout guard, including representative exact MIR ->
  pure-first EXE evidence for the allocation-apply diagnostics proof app.

## Next

Select `HAKO-ALLOC-USIZE-FIELD-GROUP-013` to close out the allocation-apply
diagnostic mirror byte field group before selecting another allocator
byte/capacity group.
