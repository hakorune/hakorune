# 293x-845 HAKO-ALLOC-USIZE-FIELD-GROUP-037 Segment-Map Consume-Ledger Release Block/Count Migration

Status: landed
Date: 2026-05-19

## Decision

Migrate the segment-map modeled consume-ledger release report block/count owner
group to exact `usize`.

This row follows the consume-ledger block/count closeout and keeps release-side
non-negative count facts separate from token and block-span sentinel fields.

## Scope

Change only these fields on
`HakoAllocSegmentMapModeledConsumeLedgerReleaseReport`:

```text
live_before
live_after
ledger_count_after
ledger_live_count_after
released_blocks
```

These are modeled non-negative release-side count facts.

## Stop Lines

- No consume-ledger main report migration in this row.
- No guarded-readiness composition, scalar lookup, page membership, or
  allocation-readiness migration in this row.
- No broad `i64` to `usize` rewrite.
- No migration of reasons, ids, indexes, tokens, block-start/end sentinels, or
  owner counters.
- No new backend route or `.inc` owner-name matcher.
- No runtime sum materialization.
- No cross-function `Result` direct ABI.
- No real raw pointer residence, real segment-map mutation, arena backing
  execution, atomic bitmap execution, OSVM/page-source execution, provider
  activation, hooks, host allocator replacement, or `#[global_allocator]`.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_release_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_release_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Notes

- Migrated only `HakoAllocSegmentMapModeledConsumeLedgerReleaseReport`
  release-side count fields to exact `usize`:
  `live_before`, `live_after`, `ledger_count_after`,
  `ledger_live_count_after`, and `released_blocks`.
- Kept reasons, ids, row indexes, tokens, block-start/end sentinels, release
  flags, and owner counters on `i64`.
- Updated the MIMAP-161A release guard typed-object contract so those five
  fields are expected as `usize` and the sentinel/token fields remain `i64`.
- Updated the historical MIMAP-162A closeout guard for current landed-card
  status drift.

## Next

`HAKO-ALLOC-USIZE-FIELD-GROUP-038` closes out the segment-map consume-ledger
release block/count group before selecting another allocator exact-`usize`
field group.
