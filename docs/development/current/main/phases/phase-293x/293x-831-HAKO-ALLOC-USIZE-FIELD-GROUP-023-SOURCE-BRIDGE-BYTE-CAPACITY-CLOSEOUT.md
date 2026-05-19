# 293x-831 HAKO-ALLOC-USIZE-FIELD-GROUP-023 Source-Bridge Byte/Capacity Closeout

Status: landed
Date: 2026-05-19

## Decision

Close out the source-bridge byte/capacity exact-`usize` field group after
`HAKO-ALLOC-USIZE-FIELD-GROUP-022`.

This row should prove that the source-bridge report byte/capacity fields are now
part of the current production `usize` storage inventory and that both the
source-bridge closeout route and the immediate source-accounting consumer remain
stable.

## Scope

- Freeze the source-bridge byte/capacity field-group migration evidence.
- Keep the group limited to:

```text
source_capacity
source_committed_bytes
requested_bytes
padded_bytes
slot_capacity
```

- Keep source-bridge diagnostic mirror byte fields on `i64`; those belong to a
  separate row.
- Keep counters, reasons, tokens, ids, alignments, row index, and
  sentinel-bearing fields on `i64`.
- Confirm `NUMERIC_FIELDS.md` lists this group as current production `usize`
  storage, not selected/pending storage.

## Stop Lines

- No second field-group migration in this row.
- No broad `i64` to `usize` rewrite.
- No source-bridge diagnostic mirror migration.
- No source-accounting, allocation-plan, allocation-apply, allocation-ledger, or
  release-candidate migration.
- No migration of counters.
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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_bridge_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_bridge_diagnostics_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_bridge_closeout_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_accounting_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Notes

- Re-ran the MIMAP-260A source-bridge L2 guard after the source-bridge
  byte/capacity migration.
- Re-ran the MIMAP-261A diagnostics L2 guard to confirm the source-bridge
  diagnostic mirror byte fields still remain `i64` in this closeout.
- Re-ran the MIMAP-262A closeout guard and kept the representative exact-MIR to
  pure-first EXE evidence green.
- Re-ran the immediate downstream MIMAP-264A source-accounting L2 guard.
- Confirmed `NUMERIC_FIELDS.md` lists the source-bridge byte/capacity report
  group as current production `usize` storage.

## Next

Select `HAKO-ALLOC-USIZE-FIELD-GROUP-024` for the source-bridge diagnostic
mirror byte fields that copy already-migrated source-bridge byte facts.
