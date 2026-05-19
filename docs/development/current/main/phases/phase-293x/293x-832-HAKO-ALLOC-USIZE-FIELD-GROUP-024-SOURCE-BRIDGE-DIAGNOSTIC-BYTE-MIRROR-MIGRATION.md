# 293x-832 HAKO-ALLOC-USIZE-FIELD-GROUP-024 Source-Bridge Diagnostic Byte Mirror Migration

Status: landed
Date: 2026-05-19

## Decision

Migrate the source-bridge diagnostic mirror byte fields to exact `usize`.

This row only updates observer copies of source-bridge byte/capacity facts that
were already migrated by `HAKO-ALLOC-USIZE-FIELD-GROUP-022` and closed out by
`HAKO-ALLOC-USIZE-FIELD-GROUP-023`.

## Scope

Change only these fields on
`HakoAllocSegmentArenaBackingModeledSourceBridgeDiagnosticReport`:

```text
last_report_source_capacity
last_report_source_committed_bytes
```

These fields are observer mirrors of already-migrated source-bridge byte facts.
They do not define a new allocator behavior boundary.

## Stop Lines

- No source-bridge report migration in this row.
- No source-accounting, allocation-plan, allocation-apply, allocation-ledger, or
  release-candidate migration in this row.
- No broad `i64` to `usize` rewrite.
- No migration of diagnostic counters.
- No migration of reason/status/flag vocabularies.
- No migration of token/id/generation fields.
- No migration of alignment fields.
- No migration of any sentinel-bearing field.
- No new backend route or `.inc` owner-name matcher.
- No runtime sum materialization.
- No cross-function `Result` direct ABI.
- No real raw pointer residence, arena backing execution, segment-map mutation,
  atomic bitmap execution, OSVM/page-source execution, provider activation,
  hooks, host allocator replacement, or `#[global_allocator]`.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_bridge_diagnostics_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_bridge_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Notes

- Migrated only the source-bridge diagnostic mirror byte fields to exact
  `usize` storage.
- Kept diagnostic counters, reasons, tokens, ids, alignments, and
  sentinel-bearing fields on `i64`.
- Strengthened the MIMAP-261A diagnostics guard to assert exact `usize`
  typed-object storage for the migrated mirror fields.
- Re-ran the MIMAP-262A closeout guard after the migration.

## Next

Select `HAKO-ALLOC-USIZE-FIELD-GROUP-025` to close out the source-bridge
diagnostic byte mirror field group before selecting another allocator
byte/capacity group.
