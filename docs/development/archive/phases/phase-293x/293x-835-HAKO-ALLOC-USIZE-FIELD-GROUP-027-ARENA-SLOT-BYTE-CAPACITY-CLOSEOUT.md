# 293x-835 HAKO-ALLOC-USIZE-FIELD-GROUP-027 Arena-Slot Byte/Capacity Closeout

Status: landed
Date: 2026-05-19

## Decision

Close out the arena-slot byte/capacity exact-`usize` field group after
`HAKO-ALLOC-USIZE-FIELD-GROUP-026`.

This row should prove that the arena-slot report byte/capacity fields are now
part of the current production `usize` storage inventory and that the
arena-slot closeout route plus immediate source-bridge consumer remain stable.

## Scope

- Freeze the arena-slot byte/capacity field-group migration evidence.
- Keep the group limited to:

```text
requested_bytes
padded_bytes
slot_capacity
```

- Keep counters, reasons, tokens, ids, alignments, geometry fields, slot index,
  row index, and sentinel-bearing fields on `i64`.
- Confirm `NUMERIC_FIELDS.md` lists this group as current production `usize`
  storage, not selected/pending storage.

## Stop Lines

- No second field-group migration in this row.
- No broad `i64` to `usize` rewrite.
- No source-bridge, source-accounting, allocation-plan, allocation-apply,
  allocation-ledger, or release-candidate migration.
- No migration of counters.
- No migration of reason/status/flag vocabularies.
- No migration of token/id/generation fields.
- No migration of alignment, slice count, page size, or geometry fields.
- No migration of `slot_index`, `row_index`, or any `-1` sentinel-bearing field.
- No new backend route or `.inc` owner-name matcher.
- No runtime sum materialization.
- No cross-function `Result` direct ABI.
- No real raw pointer residence, arena backing execution, segment-map mutation,
  atomic bitmap execution, OSVM/page-source execution, provider activation,
  hooks, host allocator replacement, or `#[global_allocator]`.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_arena_slot_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_arena_slot_diagnostics_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_arena_slot_closeout_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_bridge_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Notes

- Re-ran the MIMAP-256A arena-slot L2 guard after the arena-slot
  byte/capacity migration.
- Re-ran the MIMAP-257A diagnostics L2 guard and the MIMAP-258A closeout L3
  guard to keep the representative exact-MIR evidence green.
- Re-ran the downstream MIMAP-260A source-bridge L2 guard because source-bridge
  consumes the arena-slot byte/capacity facts.
- Confirmed `NUMERIC_FIELDS.md` lists the arena-slot byte/capacity fields as
  current production `usize` storage.
- Kept counters, reasons, tokens, ids, alignments, geometry fields, slot index,
  row index, and sentinel-bearing fields on `i64`.

## Next

Select `HAKO-ALLOC-USIZE-FIELD-GROUP-028` for the residence arena-binding
geometry count / page-size group that feeds the already-migrated arena-slot
family. This is intentionally not a byte/capacity row.
