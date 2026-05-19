# 293x-856 HAKO-ALLOC-USIZE-FIELD-GROUP-048 Local-Free Reuse Ledger Release-Apply Shape/Lookup Reject Counter Closeout

Status: selected current
Date: 2026-05-19

## Decision

Close out the modeled local-free reuse ledger release-apply shape/lookup reject
counter exact-`usize` group before selecting another allocator numeric
stored-field group.

This row is evidence-only for the four per-reason counters migrated by
`HAKO-ALLOC-USIZE-FIELD-GROUP-047`.

## Scope

Confirm that these fields on
`HakoAllocSegmentAllocationModeledLocalFreeReuseLedger` remain exact `usize`:

```text
release_apply_upstream_reject_count
release_apply_invalid_shape_reject_count
release_apply_duplicate_reject_count
release_apply_missing_reject_count
```

Confirm that execution/capability release-apply reject counters, reasons,
indexes, tokens, segment/page ids, reused block ids, flags, and sentinels stay
on signed `i64` lanes until their own narrow field-group row selects them.

## Stop Lines

- No additional stored field migration in this row.
- No migration of execution/capability release-apply reject counters in this row.
- No migration of main reuse ledger per-reason counters in this row.
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

## Next

After closeout, select the next narrow allocator exact-`usize` field group from
`lang/src/hako_alloc/memory/NUMERIC_FIELDS.md`, likely the remaining
release-apply execution/capability reject counter group.
