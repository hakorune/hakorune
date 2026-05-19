# 293x-848 HAKO-ALLOC-USIZE-FIELD-GROUP-040 Local-Free Reuse Ledger Count Migration

Status: landed
Date: 2026-05-19

## Decision

Migrate the modeled local-free reuse ledger report count group to exact
`usize`.

This row follows the segment-map consume-ledger release block/count closeout
and keeps the next migration owner-local to the existing local-free reuse
ledger report.

## Scope

Change only these fields on
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

These are modeled non-negative reuse/page/ledger count facts.

## Stop Lines

- No local-free reuse ledger release-apply report migration in this row.
- No local-free page-apply report migration in this row.
- No broad `i64` to `usize` rewrite.
- No migration of reasons, row indexes, existing indexes, tokens, segment/page
  ids, reused block ids, presence flags, capability flags, or owner counters.
- No new backend route or `.inc` owner-name matcher.
- No runtime sum materialization.
- No cross-function `Result` direct ABI.
- No real raw pointer residence, real segment-map mutation, arena backing
  execution, atomic bitmap execution, OSVM/page-source execution, provider
  activation, hooks, host allocator replacement, or `#[global_allocator]`.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Notes

- Migrated only `HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerReport`
  reuse/page/ledger count fields to exact `usize`:
  `page_used_before_reuse`, `page_used_after_reuse`,
  `page_local_free_before_reuse`, `page_local_free_after_reuse`,
  `collect_count_after_reuse`, `ledger_count_after`, and
  `ledger_live_count_after`.
- Kept reasons, row indexes, existing indexes, tokens, segment/page ids,
  reused block ids, presence flags, capability flags, and owner counters on
  `i64`.
- Updated the MIMAP-130A guard typed-object contract so those seven fields are
  expected as `usize` and the selected sentinel/id fields remain `i64`.

## Next

`HAKO-ALLOC-USIZE-FIELD-GROUP-041` closes out the modeled local-free reuse
ledger count group before selecting another allocator exact-`usize` field
group.
