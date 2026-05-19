# 293x-792 MIMAP-267A Post-Segment-Arena-Backing-ReportFields-Pilot Row Selection

Status: landed
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

## Landed Decision

Selected:

```text
MIMAP-268A
  segment arena backing modeled allocation plan inventory
```

Rationale:

- MIMAP-264A/265A/266A proved source-accounting facts and diagnostics.
- HAKO-ALLOC-REPORT-RECORD-004 reduced report-carrier debt without opening
  record pass/return/store escape.
- The next smallest allocator behavior is a model-only allocation plan that
  consumes an accepted source-accounting report and publishes planned backing
  bytes, remaining capacity, and a plan token before any real arena allocation.

Carried stop lines:

- No real raw pointer residence.
- No pointer-derived lookup or dereference.
- No real arena backing allocation.
- No real segment-map mutation.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app, box, owner, or row name.
