# 293x-553 MIMAP-066A Post-Scheduler-Marker Row Selection

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-066A` is the planning row selected by `MIMAP-065A`.

The scalar reclaim lane and the scheduler boundary/request marker slice are now
guarded. This row must choose exactly one follow-up before broader allocator
or Hakorune language work continues.

## Candidate Rows

| Candidate | Shape | Notes |
| --- | --- | --- |
| `MIMAP-067A` | allocator behavior | open one narrow next reclaim behavior after scheduler marker closeout |
| `MIMAP-SCHED-*` | substrate / scheduler | only if a real scheduler substrate row is explicitly accepted |
| `LANG-*` | Hakorune language feature | only if current allocator work should pause for source-language capability |
| `MIR-*` | compiler acceptance sidecar | only if a proof app exposes a concrete compiler acceptance blocker |

## Stop Lines

- No allocator behavior in this selection row.
- No real thread scheduling.
- No source-level concurrency feature change.
- No page-source call.
- No OSVM unreserve / release.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.
- No backend `.inc` app/name matcher.
- No cleanup bundle.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `066A.1` | Read scheduler marker closeout evidence. | closed row set is accurate. | no code |
| `066A.2` | Decide one next row. | candidate is named with stop lines. | no bundle |
| `066A.3` | Update taskboard/current pointers. | current pointer guard passes. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Selection Result

`MIMAP-066A` selects `MIMAP-067A`.

```text
row:
  MIMAP-067A reclaim scheduler substrate proposal-or-park

classification:
  allocator substrate planning row

why now:
  the scheduler marker is guarded, but real scheduling is a larger substrate
  boundary. Before implementing it, decide whether the current mimalloc lane
  should open a narrow allocator-internal scheduler substrate row, park it, or
  switch to a Hakorune language/compiler prerequisite.

stop lines:
  no allocator behavior
  no real thread scheduling
  no source-level concurrency feature change
  no page-source call
  no OSVM unreserve / release
  no provider activation
  no host allocator replacement
  no cleanup bundle
```
