# 293x-849 HAKO-ALLOC-USIZE-FIELD-GROUP-041 Local-Free Reuse Ledger Count Closeout

Status: selected current
Date: 2026-05-19

## Decision

Close out the modeled local-free reuse ledger count exact `usize` field group.

This row keeps the local-free reuse ledger migration bounded before selecting
another allocator stored-field group.

## Scope

- Confirm `HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerReport`
  reuse/page/ledger count fields remain exact `usize`:
  `page_used_before_reuse`, `page_used_after_reuse`,
  `page_local_free_before_reuse`, `page_local_free_after_reuse`,
  `collect_count_after_reuse`, `ledger_count_after`, and
  `ledger_live_count_after`.
- Confirm reasons, row indexes, existing indexes, tokens, segment/page ids,
  reused block ids, presence flags, capability flags, and owner counters remain
  `i64`.
- Preserve the existing MIMAP-130A L2 guard and MIMAP-132A closeout guard
  evidence.

## Stop Lines

- No new stored-field migration in this row.
- No broad `i64` to `usize` rewrite.
- No local-free reuse ledger release-apply report migration in this row.
- No local-free page-apply report migration in this row.
- No migration of reasons, indexes, tokens, ids, reused block ids, flags, or
  owner counters.
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

## Next

After closeout, select `HAKO-ALLOC-USIZE-FIELD-GROUP-042` to choose the next
narrow allocator exact-`usize` field group.
