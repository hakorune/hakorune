# 293x-560 MIMAP-073A Post-Scheduler-Consume Row Selection

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-073A` is the planning row selected by `MIMAP-072A`.

The scalar scheduler request ledger record/consume lifecycle is now guarded.
This row must choose exactly one follow-up before broader allocator behavior,
real scheduler substrate work, or Hakorune language work continues.

## Candidate Rows

| Candidate | Shape | Notes |
| --- | --- | --- |
| `MIMAP-074A` | allocator behavior | open one narrow next scalar allocator behavior after consume closeout |
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
| `073A.1` | Read scheduler ledger consume closeout evidence. | closed row set is accurate. | no code |
| `073A.2` | Decide one next row. | candidate is named with stop lines. | no bundle |
| `073A.3` | Update taskboard/current pointers. | current pointer guard passes. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Evidence Review

The reclaim scheduler request chain is now guarded through:

```text
MIMAP-063A scheduler boundary inventory
MIMAP-064A scheduler request marker contract
MIMAP-065A scheduler marker closeout guard
MIMAP-068A scheduler request ledger route
MIMAP-069A scheduler request ledger closeout guard
MIMAP-071A scheduler request ledger consume route
MIMAP-072A scheduler ledger consume closeout guard
```

`MIMAP-071A` proves that a pending scheduler request can be consumed locally
by page id. The record and consume rows are intentionally separate, so the next
small allocator behavior is to compose them into a single scalar roundtrip
owner before any real scheduler substrate is opened.

Opening real scheduler substrate would still cross worker handoff / progress
semantics. No proof app currently exposes a source-language or compiler
acceptance blocker.

## Selection Result

`MIMAP-073A` selects `MIMAP-074A`.

```text
row:
  MIMAP-074A reclaim scheduler request ledger roundtrip route

classification:
  allocator behavior / scalar ledger lifecycle

why now:
  after record and consume are independently guarded, the next narrow behavior
  is to prove record -> consume as one allocator-owned scalar lifecycle.
  This keeps the scheduler request model local and does not execute a real
  scheduler.

why not real scheduler substrate:
  worker handoff, run/progress semantics, and source concurrency are still
  broader substrate work. The allocator evidence can advance without them.

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
current blocker moves to MIMAP-074A.
```
