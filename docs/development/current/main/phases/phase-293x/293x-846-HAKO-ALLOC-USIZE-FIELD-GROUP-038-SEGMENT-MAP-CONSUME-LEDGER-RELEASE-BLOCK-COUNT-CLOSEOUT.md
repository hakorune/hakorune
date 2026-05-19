# 293x-846 HAKO-ALLOC-USIZE-FIELD-GROUP-038 Segment-Map Consume-Ledger Release Block/Count Closeout

Status: selected current
Date: 2026-05-19

## Decision

Close out the segment-map modeled consume-ledger release block/count exact
`usize` field group.

This row keeps the release-side count migration bounded before selecting any
new allocator stored-field group.

## Scope

- Confirm `HakoAllocSegmentMapModeledConsumeLedgerReleaseReport` release-side
  block/count fields remain exact `usize`:
  `live_before`, `live_after`, `ledger_count_after`,
  `ledger_live_count_after`, and `released_blocks`.
- Confirm reasons, ids, indexes, tokens, block-start/end sentinels, release
  flags, and owner counters remain `i64`.
- Preserve the existing MIMAP-161A L2 guard and MIMAP-162A representative L3
  closeout evidence.

## Stop Lines

- No new stored-field migration in this row.
- No broad `i64` to `usize` rewrite.
- No migration of reasons, ids, indexes, tokens, block-start/end sentinels,
  release flags, or owner counters.
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

## Next

After closeout, select `HAKO-ALLOC-USIZE-FIELD-GROUP-039` to choose the next
narrow allocator exact-`usize` field group.
