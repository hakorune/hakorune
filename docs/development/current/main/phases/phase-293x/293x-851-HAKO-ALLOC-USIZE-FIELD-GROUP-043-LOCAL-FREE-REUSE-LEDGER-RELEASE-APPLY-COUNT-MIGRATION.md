# 293x-851 HAKO-ALLOC-USIZE-FIELD-GROUP-043 Local-Free Reuse Ledger Release-Apply Count Migration

Status: selected current
Date: 2026-05-19

## Decision

Migrate the modeled local-free reuse ledger release-apply report count group to
exact `usize`.

This row follows the local-free reuse ledger count closeout and keeps the next
migration in the same owner, limited to release-apply count facts.

## Scope

Change only these fields on
`HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerReleaseApplyReport`:

```text
release_apply_count_after
release_apply_reject_count_after
ledger_live_count_after
```

These are modeled non-negative release-apply/ledger count facts.

## Stop Lines

- No local-free reuse ledger main report migration in this row.
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
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_apply_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_apply_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

After migration, select `HAKO-ALLOC-USIZE-FIELD-GROUP-044` to close out the
modeled local-free reuse ledger release-apply count group before selecting
another allocator exact-`usize` field group.
