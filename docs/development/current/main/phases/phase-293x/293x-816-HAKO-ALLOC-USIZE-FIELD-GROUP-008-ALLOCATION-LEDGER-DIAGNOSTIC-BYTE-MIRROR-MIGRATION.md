# 293x-816 HAKO-ALLOC-USIZE-FIELD-GROUP-008 Allocation-Ledger Diagnostic Byte Mirror Migration

Status: selected current
Date: 2026-05-19

## Decision

Migrate the observer-only allocation-ledger diagnostic mirror byte fields to
exact `usize`.

This row follows `HAKO-ALLOC-USIZE-FIELD-GROUP-007`, which closed out the
allocation-ledger report byte/capacity group itself.

## Scope

Change only these fields in both the diagnostic `ReportFields` record and the
diagnostic report box:

```text
last_report_applied_backing_bytes
last_report_applied_committed_bytes
last_report_remaining_source_bytes
```

These mirror non-negative byte facts from
`HakoAllocSegmentArenaBackingModeledAllocationLedgerReport`, whose
corresponding fields are already production `usize`.

## Stop Lines

- No second owner migration in this row.
- No allocation-ledger report migration in this row.
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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_diagnostics_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

After this migration, close out the allocation-ledger diagnostic mirror field
group before selecting another allocator byte/capacity group.
