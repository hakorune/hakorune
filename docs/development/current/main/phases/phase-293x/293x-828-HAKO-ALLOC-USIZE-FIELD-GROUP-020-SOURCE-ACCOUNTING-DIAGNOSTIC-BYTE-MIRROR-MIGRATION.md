# 293x-828 HAKO-ALLOC-USIZE-FIELD-GROUP-020 Source-Accounting Diagnostic Byte Mirror Migration

Status: selected current
Date: 2026-05-19

## Decision

Migrate the source-accounting diagnostic mirror byte fields to exact `usize`.

This row only updates observer copies of source-accounting byte/capacity facts
that were already migrated by `HAKO-ALLOC-USIZE-FIELD-GROUP-018` and closed out
by `HAKO-ALLOC-USIZE-FIELD-GROUP-019`.

## Scope

Change only these fields on
`HakoAllocSegmentArenaBackingModeledSourceAccountingDiagnosticReport` and its
`ReportFields` record carrier:

```text
last_report_source_capacity
last_report_source_committed_bytes
last_report_source_uncommitted_bytes
last_report_accounted_padded_bytes
last_report_available_after_padded_bytes
```

These fields are observer mirrors of already-migrated source-accounting
byte/capacity facts. They do not define a new allocator behavior boundary.

## Stop Lines

- No source-accounting report migration in this row.
- No allocation-plan, allocation-apply, allocation-ledger, or release-candidate
  migration in this row.
- No broad `i64` to `usize` rewrite.
- No migration of diagnostic counters.
- No migration of reason/status/flag vocabularies.
- No migration of token/id/generation fields.
- No migration of `row_index` or any `-1` sentinel-bearing field.
- No new backend route or `.inc` owner-name matcher.
- No runtime sum materialization.
- No cross-function `Result` direct ABI.
- No real raw pointer residence, arena backing execution, segment-map mutation,
  atomic bitmap execution, OSVM/page-source execution, provider activation,
  hooks, host allocator replacement, or `#[global_allocator]`.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_accounting_diagnostics_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_accounting_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

After this migration, select a closeout row for the source-accounting diagnostic
byte mirror field group before selecting another allocator byte/capacity group.
