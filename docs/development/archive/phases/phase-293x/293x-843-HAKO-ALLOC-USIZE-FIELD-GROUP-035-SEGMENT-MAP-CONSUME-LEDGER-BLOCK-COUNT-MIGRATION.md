# 293x-843 HAKO-ALLOC-USIZE-FIELD-GROUP-035 Segment-Map Consume-Ledger Block/Count Migration

Status: landed
Date: 2026-05-19

## Decision

Migrate the segment-map accepted-readiness modeled consume-ledger report
block/count owner group to exact `usize`.

This row starts a new downstream-first chain after the arena-backing geometry
chain. The selected group contains non-negative modeled block/count facts owned
by `HakoAllocSegmentMapAcceptedReadinessModeledConsumeLedgerReport`.

## Scope

Change only these fields on
`HakoAllocSegmentMapAcceptedReadinessModeledConsumeLedgerReport`:

```text
old_page_used
page_capacity
request_blocks
new_page_used
remaining_blocks
ledger_count_after
ledger_live_count_after
```

These are modeled non-negative block/count facts produced after accepted
guarded readiness and modeled consume/ledger checks.

## Stop Lines

- No consume-ledger release report migration in this row.
- No guarded-readiness composition, scalar lookup, page membership, or
  allocation-readiness migration in this row.
- No broad `i64` to `usize` rewrite.
- No migration of reasons, diagnostic kinds, ids, indexes, tokens, block-start
  sentinels, or `-1` sentinel-bearing fields.
- No migration of owner counters in this row.
- No new backend route or `.inc` owner-name matcher.
- No runtime sum materialization.
- No cross-function `Result` direct ABI.
- No real raw pointer residence, real segment-map mutation, arena backing
  execution, atomic bitmap execution, OSVM/page-source execution, provider
  activation, hooks, host allocator replacement, or `#[global_allocator]`.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_map_accepted_readiness_modeled_consume_ledger_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_closeout_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_release_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Notes

- Migrated only the
  `HakoAllocSegmentMapAcceptedReadinessModeledConsumeLedgerReport` block/count
  group to exact `usize` storage.
- Kept reasons, diagnostic kinds, ids, indexes, tokens, block-start sentinels,
  and owner counters on `i64`.
- Strengthened the MIMAP-157A guard to assert exact `usize` typed-object storage
  for the selected block/count fields and to assert token / block-start
  sentinels remain `i64`.
- Re-ran the MIMAP-159A closeout guard and the MIMAP-161A release L2 guard after
  the migration.

## Next

Select `HAKO-ALLOC-USIZE-FIELD-GROUP-036` to close out the segment-map
consume-ledger block/count group before selecting another allocator
exact-`usize` field group.
