# 293x-254 M209 Lifecycle Stats Observer Surface

Status: Complete

## Purpose

M209 exposes read-only lifecycle event stats on top of the M207 lifecycle
observer and M208 reuse-priority policy counters. The row adds an observer
surface only; it does not change allocator behavior.

## Decision

Decision: accepted.

Add:

```text
lang/src/hako_alloc/memory/lifecycle_stats_observer_box.hako
```

The owner returns a `HakoAllocLifecycleStatsSnapshot` from
`HakoAllocLifecycleStatsObserverSurface.snapshot(...)`.

## Row Contract

The snapshot copies existing counters from:

```text
HakoAllocPageLifecycleInvariantObserver
HakoAllocHeapReusePriorityPolicy
```

It covers:

- lifecycle observations: observed, missing, active, retired, decommitted,
  recommitted, last page, and last state
- reuse decisions: active, recommitted-active, retired-reactivate, fresh,
  decommitted skip, missing skip, last route, and last selected page
- derived totals: observed states, reuse picks, and blocked/missing events

## Stop Lines

- Do not call lifecycle observation or reuse selection from the stats owner.
- Do not mutate heap/page/marker/page-source state.
- Do not add mutable allocator options, environment variables, CLI toggles, or
  provider/hook/process allocator replacement behavior.
- Do not change packed-record/backend/materialization stop lines.

## Acceptance

- `HakoAllocLifecycleStatsObserverSurface.snapshot(...)` returns stable counters
  after the M208 priority proof matrix.
- The stats owner stays read-only and `.inc` matcher-free.
- Pure-first EXE proof output matches the lifecycle/reuse counter matrix.
- M209 guard stays local-run / index-listed and is not added to quick/dev gates.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_lifecycle_stats_observer_surface_guard.sh
```
