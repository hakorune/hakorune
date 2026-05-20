# 293x-994 BOX-INIT-003 Segment/Worker Report New-Box Field Initializer

Status: landed
Date: 2026-05-21

## Purpose

Choose the next row after provider report-copy helpers adopted explicit
`new Box { field: expr }` construction-site initializers.

## Decision

Continue applying the explicit initializer block to large ReportFields-to-Report
helpers before adding any shorthand or wildcard copy surface.

The next sweep covers high-boilerplate model/substrate report helpers:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleRemainingExecutionPrerequisiteLedger.makeRemainingExecutionPrerequisiteLedgerReport
HakoAllocWorkerTlsPilot.makeWorkerTlsPilotReport
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleExecutionReadinessMatrixDiagnostic.makeExecutionReadinessMatrixDiagnosticReport
```

Each helper now constructs the report with explicit field initializer entries:

```hako
local result = new SomeReport {
    accepted: fields.accepted,
    reason: fields.reason
}
```

## Stop Lines

- No same-name shorthand (`fields.accepted` as a standalone initializer).
- No wildcard copy.
- No spread copy.
- No constructor named arguments.
- No report schema or expected-output changes.
- No raw pointer residence, pointer-derived lookup execution, real
  release/recycle execution, arena backing release/recycle, segment-map
  mutation, atomic bitmap execution, OSVM execution, worker scheduling, provider
  activation, host allocator replacement, hooks, `#[global_allocator]`, or
  backend matcher additions.

## Evidence

```bash
bash tools/checks/k2_wide_box_new_field_initializer_segment_worker_reports_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

BOX-INIT-004 decides whether to continue the explicit initializer sweep for the
next low-risk report helper cluster or return to the parked MIMAP-375A provider
activation follow-up.
