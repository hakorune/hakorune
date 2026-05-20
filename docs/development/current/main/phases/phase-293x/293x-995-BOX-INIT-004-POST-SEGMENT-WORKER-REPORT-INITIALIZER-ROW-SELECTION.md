# 293x-995 BOX-INIT-004 Post Segment/Worker Report Initializer Row Selection

Status: current
Date: 2026-05-21

## Purpose

Choose the next row after the segment/worker report-copy sweep adopted explicit
`new Box { field: expr }` construction-site initializers.

## Candidate A: Continue Explicit Initializer Sweep

Convert another low-risk cluster of ReportFields-to-Report helpers where the
copy body is a straight `result.field = fields.field` sequence followed by
`return result`.

Recommended next cluster:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleExecutionSupportRequirementMatrix.make...
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleExecutionSupportRequirementMatrixDiagnostic.make...
HakoAllocProviderInactiveBoundaryInventory.make...
HakoAllocOSVMPageSourcePilot.make...
HakoAllocAtomicBitmapPilot.make...
```

Validation remains L2 unless a row introduces a new route shape.

## Candidate B: Park Syntax Sugar

Stop the initializer sweep and keep same-name shorthand / wildcard copy parked.
This is preferred if the remaining helpers need conditional assignments,
post-construction mutation, or owner-specific validation that would make a
mechanical rewrite hard to review.

## Candidate C: Return To Provider Activation Follow-Up

Return to parked MIMAP-375A provider activation row selection.

Provider activation, provider calls, host allocator replacement, hooks,
`#[global_allocator]`, backend matcher additions, worker scheduling, and
source concurrency remain closed unless an explicit later row opens them.

## Recommendation

Prefer Candidate A for one more small, low-risk cluster. Do not add shorthand
or wildcard copy merely for line-count reduction; the value of explicit
initializer blocks is the construction boundary and field-set contract.
