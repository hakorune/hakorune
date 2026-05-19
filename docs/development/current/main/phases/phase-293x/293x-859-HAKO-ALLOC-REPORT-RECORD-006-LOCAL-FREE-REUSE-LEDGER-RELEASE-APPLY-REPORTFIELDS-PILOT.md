# 293x-859 HAKO-ALLOC-REPORT-RECORD-006 Local-Free Reuse Ledger Release-Apply ReportFields Pilot

Status: selected current
Date: 2026-05-19

## Decision

Add an owner-local `ReportFields` record payload for the scalar-only modeled
local-free reuse ledger release-apply report.

This is a report-carrier cleanup row. It does not replace the returned
`HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerReleaseApplyReport` box.

## Scope

Add a `record` carrier next to
`HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerReleaseApplyReport`:

```text
HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerReleaseApplyReportFields
```

Build that record in the report construction path and copy the fields into the
existing returned report box.

## Stop Lines

- No owner box migration.
- No replacement of the returned report box with a record value.
- No runtime record materialization requirement beyond the existing
  ReportFields pilot pattern.
- No additional exact-`usize` stored-field migration in this row.
- No migration of reasons, indexes, tokens, segment/page ids, reused block ids,
  flags, sentinels, or lifecycle/source ids.
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

After the pilot, select a closeout or post-pilot row before broadening
ReportFields carriers to another owner.
