# 293x-792 MIMAP-267A Post-Segment-Arena-Backing-ReportFields-Pilot Row Selection

Status: selected current
Date: 2026-05-19

## Decision

Select exactly one next allocator behavior, Hakorune core capability, or
BoxShape cleanup row after the segment arena backing ReportFields pilot.

## Inputs

```text
MIMAP-266A source accounting closeout
HAKO-ALLOC-REPORT-RECORD-003 report carrier inventory
HAKO-ALLOC-REPORT-RECORD-004 source accounting diagnostic ReportFields pilot
docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md
docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md
```

## Scope

- Review the ReportFields pilot evidence.
- Decide whether the next smallest row is:
  - the next arena-backing behavior bridge,
  - one more focused report-record cleanup,
  - a focused record-carrier compiler/language sidecar,
  - or another narrow BoxShape cleanup.
- Keep the next row narrow enough to land with one focused guard/proof bundle.

## Stop Lines

- No allocator behavior change in this planning row.
- No compiler route behavior in this planning row.
- No source syntax change.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No backend `.inc` matcher.
- No silent fallback.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
