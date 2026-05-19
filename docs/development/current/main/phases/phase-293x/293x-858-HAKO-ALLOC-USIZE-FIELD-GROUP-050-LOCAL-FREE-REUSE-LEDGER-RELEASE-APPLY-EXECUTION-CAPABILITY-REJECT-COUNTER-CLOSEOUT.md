# 293x-858 HAKO-ALLOC-USIZE-FIELD-GROUP-050 Local-Free Reuse Ledger Release-Apply Execution/Capability Reject Counter Closeout

Status: selected current
Date: 2026-05-19

## Decision

Close out the modeled local-free reuse ledger release-apply execution/capability
reject counter exact-`usize` group.

This row is evidence-only for the nine per-reason counters migrated by
`HAKO-ALLOC-USIZE-FIELD-GROUP-049`.

## Scope

Confirm that these fields on
`HakoAllocSegmentAllocationModeledLocalFreeReuseLedger` remain exact `usize`:

```text
release_apply_execution_reject_count
release_apply_raw_pointer_reject_count
release_apply_segment_map_reject_count
release_apply_arena_reject_count
release_apply_atomic_bitmap_reject_count
release_apply_osvm_reject_count
release_apply_thread_reject_count
release_apply_provider_reject_count
release_apply_backend_matcher_reject_count
```

Confirm that last sentinels, reasons, indexes, tokens, segment/page ids, reused
block ids, and flags stay on signed `i64` lanes until their own narrow
field-group row selects them.

## Stop Lines

- No additional stored field migration in this row.
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

After closeout, return to the near-term task order: select a narrow
report-carrier record row for the scalar-only release-apply report, unless a
new exact numeric blocker appears first.
