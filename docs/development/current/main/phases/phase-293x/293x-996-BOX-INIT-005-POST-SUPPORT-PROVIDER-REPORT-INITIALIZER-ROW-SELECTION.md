# 293x-996 BOX-INIT-005 Post Support/Provider Report Initializer Row Selection

Status: current
Date: 2026-05-21

## Purpose

Choose the next row after support/provider report-copy helpers adopted explicit
`new Box { field: expr }` construction-site initializers.

## Candidate A: Continue Explicit Initializer Sweep

```text
Convert another low-risk cluster where:
  - `local result = new SomeReport()` is followed only by
    `result.field = fields.field`
  - the helper returns `result`
  - no conditional mutation or post-construction adjustment is needed
```

Avoid risky helpers with conditional assignments until a separate review row.

## Candidate B: Park Syntax Cleanup

Keep same-name shorthand / wildcard copy parked and return to the allocator
implementation lane.

## Candidate C: Return To Provider Activation Follow-Up

Return to parked MIMAP-375A provider activation row selection.

Provider activation, provider calls, host allocator replacement, hooks,
`#[global_allocator]`, backend matcher additions, worker scheduling, and
source concurrency remain closed unless an explicit later row opens them.

## Recommendation

Prefer Candidate B unless there is an obvious low-risk cluster. Do not add
same-name shorthand or wildcard copy for line-count reduction; explicit
initializer blocks are here to make construction boundaries and field-set
contracts visible.
