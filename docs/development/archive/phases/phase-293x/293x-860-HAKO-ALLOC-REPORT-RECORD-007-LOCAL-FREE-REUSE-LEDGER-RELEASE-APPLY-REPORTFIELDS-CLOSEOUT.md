# 293x-860 HAKO-ALLOC-REPORT-RECORD-007 Local-Free Reuse Ledger Release-Apply ReportFields Closeout

Status: landed
Date: 2026-05-19

## Decision

Close out the owner-local `ReportFields` record carrier pilot for the modeled
local-free reuse ledger release-apply report.

This row is evidence-only for `HAKO-ALLOC-REPORT-RECORD-006`.

## Scope

Confirm that:

```text
HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerReleaseApplyReportFields
```

is present as a local record declaration, is constructed only inside the
release-apply report construction paths, and is copied into the existing returned
`HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerReleaseApplyReport` box.

## Stop Lines

- No owner box migration.
- No replacement of the returned report box with a record value.
- No passing ReportFields records through helper calls.
- No additional exact-`usize` stored-field migration in this row.
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

This closeout deliberately kept `ReportFields` local to the constructing
function. During the pilot, passing the builder-local record value through a
helper call failed fast with:

```text
[record-value/escape] ... supported_use=field-read
```

That rejection is correct for C205b: record values are currently builder-local
scalarization carriers, not runtime objects. The next row therefore must not
relax the escape guard by itself. It must add an explicit helper-argument
scalarization contract before allocator owners can factor repeated report-copy
boilerplate into helpers.

Selected next:

```text
RECORD-VALUE-HELPER-001
```
