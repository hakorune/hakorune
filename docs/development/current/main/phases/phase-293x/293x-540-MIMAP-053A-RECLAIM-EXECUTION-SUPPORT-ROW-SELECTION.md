# 293x-540 MIMAP-053A Reclaim Execution Support Row Selection

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-053A` is the planning row selected by `MIMAP-052B`.

Reclaim execution intent is now MIR-visible as `hako.alloc.reclaim`, and the
pure-first preflight can reject unsupported reclaim execution before backend
emission. The next step must choose exactly one implementation row before any
owner-transfer execution opens.

## Scope

- Read `MIMAP-051A` owner-transfer contract evidence.
- Read `MIMAP-052B` reclaim execution marker/preflight evidence.
- Decide whether the next implementation row should be:
  - a first guarded reclaim execution slice;
  - an atomic-claim contract sidecar;
  - a remote-free drain fail-fast row;
  - or another no-execution allocator row.
- Update current pointers and taskboard after selection.

## Stop Lines

- No reclaim execution.
- No owner mutation.
- No atomic claim.
- No remote-free drain.
- No thread scheduling.
- No page-source call.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.
- No cleanup bundle.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `053A.1` | Read marker/preflight and owner-transfer evidence. | missing execution prerequisite is classified. | no implementation |
| `053A.2` | Select exactly one next row. | one token is named. | no bundle |
| `053A.3` | Update current pointers and taskboard. | current pointer guard passes. | no landed history expansion |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
