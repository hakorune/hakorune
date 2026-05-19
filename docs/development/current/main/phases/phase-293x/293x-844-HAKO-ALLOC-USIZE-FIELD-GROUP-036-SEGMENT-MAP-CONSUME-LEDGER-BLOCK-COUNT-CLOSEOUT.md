# 293x-844 HAKO-ALLOC-USIZE-FIELD-GROUP-036 Segment-Map Consume-Ledger Block/Count Closeout

Status: selected current
Date: 2026-05-19

## Decision

Close out the segment-map accepted-readiness modeled consume-ledger block/count
exact-`usize` field group after `HAKO-ALLOC-USIZE-FIELD-GROUP-035`.

This row should prove that the consume-ledger block/count fields are now part of
the current production `usize` storage inventory and that the consume-ledger
closeout route plus immediate release consumer remain stable.

## Scope

- Freeze the segment-map consume-ledger block/count field-group migration
  evidence.
- Keep the group limited to:

```text
old_page_used
page_capacity
request_blocks
new_page_used
remaining_blocks
ledger_count_after
ledger_live_count_after
```

- Keep reasons, diagnostic kinds, ids, indexes, tokens, block-start sentinels,
  and owner counters on `i64`.
- Confirm `NUMERIC_FIELDS.md` lists this group as current production `usize`
  storage, not selected/pending storage.

## Stop Lines

- No second field-group migration in this row.
- No broad `i64` to `usize` rewrite.
- No consume-ledger release report migration.
- No guarded-readiness composition, scalar lookup, page membership, or
  allocation-readiness migration.
- No migration of reasons, diagnostic kinds, ids, indexes, tokens, block-start
  sentinels, or owner counters.
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

## Next

After this closeout, select the next allocator exact-`usize` field group.
