# 293x-563 MIMAP-076A Post-Scheduler-Roundtrip Row Selection

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-076A` is the planning row selected by `MIMAP-075A`.

The scalar scheduler request ledger record/consume/roundtrip lifecycle is now
guarded. This row must choose exactly one follow-up before broader allocator
behavior, real scheduler substrate work, or Hakorune language work continues.

## Candidate Rows

| Candidate | Shape | Notes |
| --- | --- | --- |
| `MIMAP-077A` | allocator behavior | open one narrow next scalar allocator behavior after roundtrip closeout |
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
| `076A.1` | Read scheduler roundtrip closeout evidence. | closed row set is accurate. | no code |
| `076A.2` | Decide one next row. | candidate is named with stop lines. | no bundle |
| `076A.3` | Update taskboard/current pointers. | current pointer guard passes. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
