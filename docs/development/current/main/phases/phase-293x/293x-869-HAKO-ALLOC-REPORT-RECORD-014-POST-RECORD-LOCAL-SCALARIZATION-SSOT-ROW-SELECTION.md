# 293x-869 HAKO-ALLOC-REPORT-RECORD-014 Post Record-Local Scalarization SSOT Row Selection

Status: landed
Date: 2026-05-20

## Decision

Select the next single allocator `ReportFields` owner after the record-local
scalarization SSOT was fixed.

SSOT:

```text
docs/development/current/main/design/record-local-scalarization-ssot.md
```

## Scope

- Inventory nearby `ReportFields` owners.
- Select exactly one owner or decide to pause `ReportFields` migration and
  return to the allocator modeled lane.
- Keep the selected row bounded to the SSOT stop lines.

## Stop Lines

- No new owner migration in this selection row.
- No broad conversion from report boxes to records.
- No record return values.
- No runtime record representation, packed storage, or backend matcher.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No real raw pointer residence, real segment-map mutation, arena backing
  execution, atomic bitmap execution, OSVM/page-source execution, provider
  activation, hooks, host allocator replacement, or `#[global_allocator]`.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Completion Criteria

- The next row is selected with a single owner and a bounded validation profile.
- The selected row references the record-local scalarization SSOT.

## Inventory

Nearby `ReportFields` owners still using inline report construction:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerDiagnosticReportFields
HakoAllocSegmentArenaBackingModeledAllocationApplyDiagnosticReportFields
HakoAllocSegmentArenaBackingModeledAllocationPlanDiagnosticReportFields
HakoAllocSegmentArenaBackingModeledSourceAccountingDiagnosticReportFields
HakoAllocSegmentAllocationModeledLocalFreeIntegrationReportFields
HakoAllocBoundedPurgeDecommitSchedulerReportFields
```

Already migrated through helper-argument scalarization:

```text
HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerReleaseApplyReportFields
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateReportFields
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateDiagnosticReportFields
```

## Selection

Select `HAKO-ALLOC-REPORT-RECORD-015`:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerDiagnosticReportFields
```

Reason:

```text
It is adjacent to the already migrated allocation-ledger release-candidate
diagnostic owner, uses the same scalar-only diagnostic report style, and can
stay inside the record-local scalarization SSOT without opening a broader
helper body shape.
```

## Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
