# 293x-854 HAKO-ALLOC-USIZE-FIELD-GROUP-046 Local-Free Reuse Ledger Release-Apply Primary Counter Closeout

Status: landed
Date: 2026-05-19

## Decision

Close out the modeled local-free reuse ledger release-apply primary counter
exact-`usize` group before selecting another allocator numeric stored-field
group.

This row is evidence-only for the three owner-local counters migrated by
`HAKO-ALLOC-USIZE-FIELD-GROUP-045`.

## Scope

Confirm that these fields on
`HakoAllocSegmentAllocationModeledLocalFreeReuseLedger` remain exact `usize`:

```text
release_apply_attempt_count
release_apply_count
release_apply_reject_count
```

Confirm that per-reason release-apply reject counters, reasons, indexes, tokens,
segment/page ids, reused block ids, flags, and sentinels stay on signed `i64`
lanes until their own narrow field-group row selects them.

## Stop Lines

- No additional stored field migration in this row.
- No migration of per-reason release-apply reject counters in this row.
- No migration of the main reuse ledger owner counters in this row.
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

- Closed out the three-field release-apply primary counter group migrated by
  `HAKO-ALLOC-USIZE-FIELD-GROUP-045`.
- Reconfirmed that per-reason reject counters and signed sentinels remain on
  `i64`.
- Selected `HAKO-ALLOC-USIZE-FIELD-GROUP-047` for the first narrow
  release-apply per-reason reject counter group.

## Next

`HAKO-ALLOC-USIZE-FIELD-GROUP-047` migrates only the upstream / invalid-shape /
duplicate / missing release-apply reject counters.
