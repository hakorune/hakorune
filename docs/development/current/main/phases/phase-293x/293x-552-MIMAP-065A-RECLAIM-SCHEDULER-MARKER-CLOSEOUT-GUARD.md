# 293x-552 MIMAP-065A Reclaim Scheduler Marker Closeout Guard

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-065A` is the closeout guard row selected by `MIMAP-064A`.

The row should lock the scheduler boundary and request marker rows before any
future real scheduling or broader reclaim behavior is considered.

## Scope

- Add a scheduler marker closeout SSOT.
- Add a focused guard that checks `MIMAP-063A` and `MIMAP-064A` coverage.
- Keep real scheduling, source-level concurrency, page-source calls,
  OSVM release, provider activation, and host allocator replacement closed.
- Select the next row after closeout.

## Stop Lines

- No new `.hako` allocator owner behavior.
- No real thread scheduling.
- No source-level `nowait`, `Channel`, `task_scope`, `co`, `sync box`,
  `context`, or `worker_local` semantics.
- No page-source call.
- No OSVM unreserve / release.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.
- No backend `.inc` app/name matcher.
- No cleanup bundle.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `065A.1` | Write scheduler marker closeout SSOT. | landed row set and inactive surfaces fixed. | no behavior |
| `065A.2` | Add closeout guard. | boundary + request marker proof are verified. | no scheduling |
| `065A.3` | Update docs index and current pointers. | current pointer guard passes. | no bundle |
| `065A.4` | Select follow-up row. | next row has one narrow owner. | no feature mixing |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_reclaim_scheduler_marker_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Implementation Result

`MIMAP-065A` added the scheduler marker closeout SSOT and guard.

The guard locks `MIMAP-063A` and `MIMAP-064A`, including the accepted boundary
and marker SSOTs, the proof app manifest entry, focused guard index rows,
module export, memory README owner description, `.inc` no-growth, provider
inactive sentinel, and source-concurrency/scheduling stop lines.

## Evidence

```text
bash tools/checks/k2_wide_hako_alloc_reclaim_scheduler_marker_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Selection Result

`MIMAP-065A` selects `MIMAP-066A`.

```text
row:
  MIMAP-066A post-scheduler-marker row selection

classification:
  planning row

why now:
  scheduler boundary and marker are closed. The next row should decide one
  narrow follow-up: continue allocator reclaim, open a concrete compiler /
  language sidecar, or switch to a broader Hakorune language feature lane.

stop lines:
  no new allocator behavior
  no real thread scheduling
  no source-level concurrency feature change
  no page-source call
  no OSVM unreserve / release
  no provider activation
  no host allocator replacement
  no cleanup bundle
```
