# 293x-881 HAKO-ALLOC-REPORT-RECORD-026 Post Source-Accounting Diagnostic ReportFields Helper Scalarization Closeout Row Selection

Status: landed
Date: 2026-05-20

## Decision

Select the next narrow row after closing the source-accounting diagnostic
`ReportFields` helper-scalarization owner.

SSOT:

```text
docs/development/current/main/design/record-local-scalarization-ssot.md
```

## Scope

- Inventory the remaining `ReportFields` owners.
- Decide whether to migrate exactly one more owner or pause `ReportFields`
  migration and return to the allocator modeled lane.
- Keep the selected row bounded to the record-local scalarization SSOT.

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

- The next row is selected with a single owner and a bounded validation profile,
  or `ReportFields` migration is explicitly paused.
- The selected row references the record-local scalarization SSOT.

## Inventory

Remaining `ReportFields` owners:

```text
HakoAllocSegmentAllocationModeledLocalFreeIntegrationReportFields
HakoAllocBoundedPurgeDecommitSchedulerReportFields
```

Already migrated through helper-argument scalarization:

```text
HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerReleaseApplyReportFields
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateReportFields
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateDiagnosticReportFields
HakoAllocSegmentArenaBackingModeledAllocationLedgerDiagnosticReportFields
HakoAllocSegmentArenaBackingModeledAllocationApplyDiagnosticReportFields
HakoAllocSegmentArenaBackingModeledAllocationPlanDiagnosticReportFields
HakoAllocSegmentArenaBackingModeledSourceAccountingDiagnosticReportFields
```

## Selection

Select `HAKO-ALLOC-REPORT-RECORD-027`:

```text
HakoAllocSegmentAllocationModeledLocalFreeIntegrationReportFields
```

Reason:

```text
It is the widest remaining live allocator report carrier and has four repeated
ordinary report construction sites. Migrating it removes the largest remaining
ReportFields duplication while staying inside the record-local scalarization
SSOT. The scheduler report is smaller and can remain for a later selection.
```

## Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
