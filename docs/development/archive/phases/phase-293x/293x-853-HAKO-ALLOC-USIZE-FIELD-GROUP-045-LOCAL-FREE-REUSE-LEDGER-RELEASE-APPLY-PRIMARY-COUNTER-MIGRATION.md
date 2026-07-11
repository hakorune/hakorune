# 293x-853 HAKO-ALLOC-USIZE-FIELD-GROUP-045 Local-Free Reuse Ledger Release-Apply Primary Counter Migration

Status: landed
Date: 2026-05-19

## Decision

Migrate the modeled local-free reuse ledger release-apply owner primary counters
to exact `usize`.

This row follows the release-apply report count closeout and keeps the migration
inside the same owner, limited to monotonic release-apply counter facts.

## Scope

Change only these fields on
`HakoAllocSegmentAllocationModeledLocalFreeReuseLedger`:

```text
release_apply_attempt_count
release_apply_count
release_apply_reject_count
```

These are modeled non-negative release-apply primary counters. They feed the
already-migrated release-apply report count fields.

## Stop Lines

- No migration of per-reason release-apply reject counters in this row.
- No migration of the main reuse ledger owner counters in this row.
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

- Migrated only the three owner-local release-apply primary counters to exact
  `usize`.
- Kept all per-reason release-apply reject counters, last sentinels, reasons,
  indexes, tokens, segment/page ids, reused block ids, and flags on `i64`.
- Extended the MIMAP-138A guard to assert both the report count field contract
  and the owner primary counter typed-object contract.

## Next

`HAKO-ALLOC-USIZE-FIELD-GROUP-046` closes out the release-apply primary counter
group before selecting per-reason counters or a different allocator
exact-`usize` field group.
