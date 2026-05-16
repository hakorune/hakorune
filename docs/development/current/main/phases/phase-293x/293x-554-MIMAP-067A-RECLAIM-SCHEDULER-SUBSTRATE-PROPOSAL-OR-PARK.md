# 293x-554 MIMAP-067A Reclaim Scheduler Substrate Proposal Or Park

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-067A` is the allocator substrate planning row selected by `MIMAP-066A`.

The row decides whether to open a narrow allocator-internal scheduler substrate
implementation lane, park real scheduling for now, or switch to a concrete
Hakorune language/compiler prerequisite.

## Scope

- Read the closed scalar reclaim and scheduler marker evidence.
- Compare next options:
  - allocator-internal scheduler substrate
  - next scalar allocator behavior without real scheduling
  - Hakorune language feature prerequisite
  - compiler acceptance sidecar
- Select exactly one next row with stop lines.

## Stop Lines

- No allocator behavior in this row.
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
| `067A.1` | Read closed reclaim/scheduler evidence. | blocker set is accurate. | no code |
| `067A.2` | Decide open vs park for real scheduler substrate. | one row is selected. | no bundle |
| `067A.3` | Update current pointers. | current pointer guard passes. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
