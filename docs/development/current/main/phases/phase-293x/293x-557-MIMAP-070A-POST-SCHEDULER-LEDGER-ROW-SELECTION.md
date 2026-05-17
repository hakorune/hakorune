# 293x-557 MIMAP-070A Post-Scheduler-Ledger Row Selection

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-070A` is the planning row selected by `MIMAP-069A`.

The scalar reclaim scheduler request ledger slice is now guarded. This row must
choose exactly one follow-up before broader allocator behavior, real scheduler
substrate work, or Hakorune language work continues.

## Candidate Rows

| Candidate | Shape | Notes |
| --- | --- | --- |
| `MIMAP-071A` | allocator behavior | open one narrow next scalar allocator behavior after the ledger closeout |
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
| `070A.1` | Read scheduler ledger closeout evidence. | closed row set is accurate. | no code |
| `070A.2` | Decide one next row. | candidate is named with stop lines. | no bundle |
| `070A.3` | Update taskboard/current pointers. | current pointer guard passes. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Evidence Review

The scheduler request chain is now guarded through:

```text
MIMAP-063A scheduler boundary inventory
MIMAP-064A scheduler request marker contract
MIMAP-065A scheduler marker closeout guard
MIMAP-068A scheduler request ledger route
MIMAP-069A scheduler request ledger closeout guard
```

`MIMAP-068A` records at most one pending modeled scheduler request. It proves
recorded, already-pending, scheduler-disabled, and completion-blocked rows, but
it does not yet model the local lifecycle of clearing that pending request.

Opening real scheduler substrate would still cross worker handoff / run queue /
progress semantics. No proof app currently exposes a compiler acceptance
blocker or source-language blocker.

## Selection Result

`MIMAP-070A` selects `MIMAP-071A`.

```text
row:
  MIMAP-071A reclaim scheduler request ledger consume route

classification:
  allocator behavior / scalar ledger lifecycle

why now:
  after the request ledger can record one pending request, the next narrow
  behavior is to consume/clear that pending request locally. This proves the
  ledger lifecycle without executing real scheduling or opening source
  concurrency semantics.

why not real scheduler substrate:
  real scheduling still needs worker handoff, progress, wake/run semantics, and
  timeout diagnostics. The current allocator evidence can advance with a scalar
  consume route first.

why not language/compiler sidecar:
  no current proof is blocked on source-level concurrency features or compiler
  acceptance.

stop lines:
  no real thread scheduling
  no worker spawning
  no source-level concurrency feature change
  no page-source call
  no OSVM unreserve / release
  no provider activation
  no host allocator replacement
  no backend matcher
```

Closeout:

```text
current blocker moves to MIMAP-071A.
```
