# 293x-790 HAKO-ALLOC-REPORT-RECORD-003 Segment Arena Backing Report Record Carrier Inventory

Status: planned
Date: 2026-05-19

## Decision

After the MIMAP-266A source-accounting closeout, run one focused BoxShape row
before opening the next arena-backing behavior row.

The row inventories segment-arena-backing proof report objects whose fields are
all scalar `i64` facts and decides the smallest safe record-carrier cleanup.

## Why Here

MIMAP-260A through MIMAP-265A added source-bridge and source-accounting reports
that are semantically identity-free diagnostic payloads. Keeping them as ordinary
`box` objects is acceptable as the current stable compiler/backend carrier, but
it is not the clean final shape.

Do this immediately after the MIMAP-266A closeout so new arena-backing rows do
not accumulate more report-object debt before the next bridge.

## Scope

- Inventory all segment-arena-backing report boxes introduced or touched by:
  - MIMAP-260A modeled source bridge inventory
  - MIMAP-261A modeled source bridge diagnostics
  - MIMAP-264A modeled source accounting inventory
  - MIMAP-265A modeled source accounting diagnostics
- Classify each report as:
  - local `ReportFields` record payload candidate while keeping the returned
    report box stable,
  - blocked by record pass/return/store escape or backend support,
  - not worth changing because the scalar box is still clearer.
- Select at most one pilot cleanup row, or select a focused compiler/language
  sidecar if full record-carrier use is the real blocker.

## Stop Lines

- No allocator behavior change.
- No broad report sweep.
- No cross-function record return unless a separate compiler row explicitly
  opens it.
- No record pass/store escape.
- No packed/backend record lowering.
- No backend `.inc` matcher by app, box, or owner name.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No silent fallback.

## Acceptance

This row closes when it records one of:

```text
next:
  one owner-local ReportFields record payload pilot

or:
  one compiler/language record-carrier acceptance sidecar

or:
  explicit park reason if the current report boxes remain the smallest stable carrier
```

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
