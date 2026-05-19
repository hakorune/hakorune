# 293x-834 HAKO-ALLOC-USIZE-FIELD-GROUP-026 Arena-Slot Byte/Capacity Migration

Status: landed
Date: 2026-05-19

## Decision

Migrate the modeled arena-slot report byte/capacity owner group to exact
`usize`.

This row moves one owner upstream from source-bridge. The group contains
non-negative modeled byte/capacity facts that feed the already-migrated
source-bridge family.

## Scope

Change only these fields on
`HakoAllocSegmentArenaBackingModeledArenaSlotReport`:

```text
requested_bytes
padded_bytes
slot_capacity
```

These are modeled byte/capacity facts copied from accepted arena-slot inputs
after local validation.

## Stop Lines

- No arena-slot diagnostic mirror migration in this row.
- No source-bridge, source-accounting, allocation-plan, allocation-apply,
  allocation-ledger, or release-candidate migration in this row.
- No broad `i64` to `usize` rewrite.
- No migration of arena-slot counters.
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

- Migrated only the `HakoAllocSegmentArenaBackingModeledArenaSlotReport`
  byte/capacity group to exact `usize` storage.
- Kept arena-slot counters, reasons, tokens, ids, alignments, geometry fields,
  slot index, row index, and sentinel-bearing fields on `i64`.
- Strengthened the MIMAP-256A guard to assert exact `usize` typed-object storage
  for the arena-slot byte/capacity fields.
- Re-ran the MIMAP-257A diagnostics guard, the MIMAP-258A closeout guard, and
  the downstream MIMAP-260A source-bridge L2 guard after the migration.

## Next

Select `HAKO-ALLOC-USIZE-FIELD-GROUP-027` to close out the arena-slot
byte/capacity field group before selecting another allocator byte/capacity
group.
