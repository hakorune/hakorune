# 293x-847 HAKO-ALLOC-USIZE-FIELD-GROUP-039 Next Field-Group Selection

Status: landed
Date: 2026-05-19

## Decision

Select the next narrow allocator stored numeric field group for exact `usize`
migration after closing the segment-map consume-ledger release block/count
group.

This is a planning row. It does not migrate fields by itself.

## Scope

- Inspect the remaining hako_alloc stored numeric fields.
- Select one owner-local, non-negative size/count/capacity field group.
- Record why the selected fields are safe to migrate together.
- Keep signed sentinel, signed reason/status, ids/tokens, block-span sentinels,
  pointer-shaped handles, and counters that intentionally stay signed out of
  scope.

## Stop Lines

- No stored-field migration in this row.
- No broad `i64` to `usize` rewrite.
- No runtime sum materialization.
- No cross-function `Result` direct ABI.
- No new backend route or `.inc` owner-name matcher.
- No real raw pointer residence, real segment-map mutation, arena backing
  execution, atomic bitmap execution, OSVM/page-source execution, provider
  activation, hooks, host allocator replacement, or `#[global_allocator]`.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Notes

Selected `HAKO-ALLOC-USIZE-FIELD-GROUP-040` to migrate the modeled
local-free reuse ledger report count group on
`HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerReport`:

```text
page_used_before_reuse
page_used_after_reuse
page_local_free_before_reuse
page_local_free_after_reuse
collect_count_after_reuse
ledger_count_after
ledger_live_count_after
```

These fields are owner-local non-negative modeled reuse/page/ledger counts.
Reasons, row indexes, existing indexes, source/allocation tokens, segment/page
ids, reused block ids, presence flags, capability flags, and owner counters
remain out of scope.

## Next

`HAKO-ALLOC-USIZE-FIELD-GROUP-040` migrates the selected local-free reuse
ledger count group.
