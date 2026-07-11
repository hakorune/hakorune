# 293x-812 HAKO-ALLOC-USIZE-FIELD-GROUP-004 Release-Candidate Diagnostic Byte Mirror Migration

Status: landed
Date: 2026-05-19

## Decision

Migrate the observer-only release-candidate diagnostic mirror byte fields to
exact `usize`.

This row follows `HAKO-ALLOC-USIZE-FIELD-GROUP-003`, which closed out the first
production exact-`usize` stored field group on the release-candidate report
itself.

## Scope

Change only these fields in both the diagnostic `ReportFields` record and the
diagnostic report box:

```text
last_report_applied_backing_bytes
last_report_applied_committed_bytes
last_report_remaining_source_bytes
```

These mirror non-negative byte facts from
`HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateReport`,
whose corresponding fields are already production `usize`.

## Stop Lines

- No second owner migration in this row.
- No migration of diagnostic counters.
- No migration of reason/status/flag vocabularies.
- No migration of token/id/generation fields.
- No migration of `last_segment_id`, `last_arena_id`, `row_index`, or any
  sentinel-bearing field.
- No new backend route or `.inc` owner-name matcher.
- No runtime sum materialization.
- No cross-function `Result` direct ABI.
- No real raw pointer residence, arena backing execution, segment-map mutation,
  atomic bitmap execution, OSVM/page-source execution, provider activation,
  hooks, host allocator replacement, or `#[global_allocator]`.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_candidate_diagnostics_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_candidate_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Notes

- Migrated the three diagnostic mirror byte fields in both
  `HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateDiagnosticReportFields`
  and
  `HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateDiagnosticReport`.
- Extended the MIMAP-281A diagnostics guard to verify both record declarations
  and typed object storage for the migrated fields.
- Re-ran MIMAP-281A L2 and MIMAP-282A closeout L3 after the migration.

## Next

Select `HAKO-ALLOC-USIZE-FIELD-GROUP-005` to close out the diagnostic mirror
field group before selecting another allocator byte/capacity group.
