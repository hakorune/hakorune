# 293x-855 HAKO-ALLOC-USIZE-FIELD-GROUP-047 Local-Free Reuse Ledger Release-Apply Shape/Lookup Reject Counter Migration

Status: landed
Date: 2026-05-19

## Decision

Migrate the first narrow modeled local-free reuse ledger release-apply
per-reason reject counter group to exact `usize`.

This row follows the release-apply primary counter closeout and limits the
migration to non-negative shape / lookup reject counters.

## Scope

Change only these fields on
`HakoAllocSegmentAllocationModeledLocalFreeReuseLedger`:

```text
release_apply_upstream_reject_count
release_apply_invalid_shape_reject_count
release_apply_duplicate_reject_count
release_apply_missing_reject_count
```

These are modeled non-negative per-reason reject counters for upstream rejection,
invalid release shape, duplicate release-apply, and missing ledger entry.

## Stop Lines

- No migration of execution/capability release-apply reject counters in this row.
- No migration of main reuse ledger per-reason counters in this row.
- No migration of reasons, indexes, tokens, segment/page ids, reused block ids,
  flags, sentinels, or lifecycle/source ids.
- No broad `i64` to `usize` rewrite.
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

## Landed Notes

- Migrated only the four release-apply shape/lookup reject counters to exact
  `usize`.
- Kept execution/capability release-apply reject counters, last sentinels,
  reasons, indexes, tokens, segment/page ids, reused block ids, and flags on
  `i64`.
- Extended the MIMAP-138A guard to assert the migrated reject-counter group and
  the intentionally signed execution/capability reject counters.

## Next

`HAKO-ALLOC-USIZE-FIELD-GROUP-048` closes out the release-apply shape/lookup
reject counter group before selecting execution or capability reject counters.
