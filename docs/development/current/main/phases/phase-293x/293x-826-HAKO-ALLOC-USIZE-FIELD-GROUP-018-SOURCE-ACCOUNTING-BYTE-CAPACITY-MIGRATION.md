# 293x-826 HAKO-ALLOC-USIZE-FIELD-GROUP-018 Source-Accounting Byte/Capacity Migration

Status: landed
Date: 2026-05-19

## Decision

Migrate the modeled source-accounting report byte/capacity owner group to exact
`usize`.

This row continues moving upstream from allocation-plan to the source-accounting
owner that feeds it. The group contains non-negative modeled byte/capacity facts
only; reason/status/token/id/sentinel fields remain signed.

## Scope

Change only these fields on
`HakoAllocSegmentArenaBackingModeledSourceAccountingReport`:

```text
source_capacity
source_committed_bytes
source_uncommitted_bytes
slot_capacity
padded_bytes
accounted_padded_bytes
available_after_padded_bytes
```

These are non-negative modeled byte/capacity facts copied from accepted source
reports or computed from accepted source facts. They are already proven
downstream on the allocation-plan, allocation-apply, allocation-ledger, and
release-candidate families.

## Stop Lines

- No source-accounting diagnostic mirror migration in this row.
- No allocation-plan, allocation-apply, allocation-ledger, or release-candidate
  migration in this row.
- No migration of source-accounting counters.
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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_accounting_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_accounting_diagnostics_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_accounting_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Notes

- Migrated only the
  `HakoAllocSegmentArenaBackingModeledSourceAccountingReport` byte/capacity
  owner group to exact `usize` storage.
- Kept counters, reasons, tokens, ids, source alignment, slot index, and
  sentinel-bearing fields on `i64`.
- Strengthened the MIMAP-264A guard to assert exact `usize` typed-object
  storage for the source-accounting byte/capacity fields.
- Strengthened the MIMAP-265A diagnostics guard to prove the source-accounting
  diagnostic mirror byte fields remain `i64` in this row.
- Re-ran the MIMAP-266A closeout guard after fixing historical granularity and
  report-record next-row status expectations.

## Next

Select `HAKO-ALLOC-USIZE-FIELD-GROUP-019` to close out the source-accounting
byte/capacity field group before selecting another allocator byte/capacity
group.
