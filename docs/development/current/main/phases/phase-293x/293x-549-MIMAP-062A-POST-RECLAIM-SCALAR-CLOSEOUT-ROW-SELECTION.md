# 293x-549 MIMAP-062A Post-Reclaim-Scalar-Closeout Row Selection

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-062A` is the planning row selected by `MIMAP-061A`.

Scalar reclaim is now guarded through owner-transfer, atomic-claim,
remote-free-drain, post-drain transfer, and completion marker rows. This row
must select exactly one next implementation or language/compiler sidecar row.

## Candidate Rows

| Candidate | Shape | Notes |
| --- | --- | --- |
| `MIMAP-063A` | allocator behavior | open one narrow next reclaim behavior after scalar closeout |
| `MIMAP-SCHED-*` | substrate / scheduler | only if reclaim needs worker scheduling before broader behavior |
| `LANG-*` | Hakorune language feature | only if current allocator row is blocked by source language capability |
| `MIR-*` | compiler acceptance sidecar | only if a proof app exposes a concrete compiler acceptance blocker |

## Stop Lines

- No allocator behavior in this selection row.
- No thread scheduling.
- No page-source call.
- No OSVM unreserve / release.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.
- No backend `.inc` app/name matcher.
- No cleanup bundle.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `062A.1` | Read scalar reclaim closeout evidence. | closed row set is accurate. | no code |
| `062A.2` | Decide one next row. | candidate is named with stop lines. | no bundle |
| `062A.3` | Update taskboard/current pointers. | current pointer guard passes. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
