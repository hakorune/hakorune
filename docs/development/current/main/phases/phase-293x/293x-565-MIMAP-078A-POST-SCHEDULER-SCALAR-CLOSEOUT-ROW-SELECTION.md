# 293x-565 MIMAP-078A Post-Scheduler-Scalar-Closeout Row Selection

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-078A` is the planning row selected by `MIMAP-077A`.

The scalar scheduler lane is now closed out. This row must choose exactly one
follow-up before broader allocator behavior, real scheduler substrate work, or
Hakorune language work continues.

## Candidate Rows

| Candidate | Shape | Notes |
| --- | --- | --- |
| `MIMAP-079A` | allocator behavior | open one narrow next scalar allocator behavior after scheduler scalar closeout |
| `MIMAP-SCHED-*` | substrate / scheduler | only if real scheduler substrate is explicitly accepted |
| `LANG-*` | Hakorune language feature | only if current allocator work should pause for source-language capability |
| `MIR-*` | compiler acceptance sidecar | only if a proof app exposes a concrete compiler acceptance blocker |

## Stop Lines

- No allocator behavior in this selection row.
- No real thread scheduling.
- No worker spawning.
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
| `078A.1` | Read scheduler scalar closeout evidence. | closed row set is accurate. | no code |
| `078A.2` | Decide one next row. | candidate is named with stop lines. | no bundle |
| `078A.3` | Update taskboard/current pointers. | current pointer guard passes. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
