# 293x-563 MIMAP-076A Post-Scheduler-Roundtrip Row Selection

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-076A` is the planning row selected by `MIMAP-075A`.

The scalar scheduler request ledger record/consume/roundtrip lifecycle is now
guarded. This row must choose exactly one follow-up before broader allocator
behavior, real scheduler substrate work, or Hakorune language work continues.

## Candidate Rows

| Candidate | Shape | Notes |
| --- | --- | --- |
| `MIMAP-077A` | closeout | close the scheduler scalar lane before selecting broader work |
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

## Evidence Review

The scheduler scalar chain is now guarded through:

```text
MIMAP-063A scheduler boundary inventory
MIMAP-064A scheduler request marker contract
MIMAP-065A scheduler marker closeout guard
MIMAP-067A scheduler substrate proposal-or-park
MIMAP-068A scheduler request ledger route
MIMAP-069A scheduler request ledger closeout guard
MIMAP-071A scheduler request ledger consume route
MIMAP-072A scheduler ledger consume closeout guard
MIMAP-074A scheduler request ledger roundtrip route
MIMAP-075A scheduler ledger roundtrip closeout guard
```

Record, consume, and roundtrip are now separately guarded. Before selecting a
new allocator behavior or reopening real scheduler substrate, the lane should
lock the full scheduler scalar slice as a single closeout surface.

No proof app currently exposes a source-language or compiler acceptance
blocker.

## Selection Result

`MIMAP-076A` selects `MIMAP-077A`.

```text
row:
  MIMAP-077A reclaim scheduler scalar lane closeout guard

classification:
  closeout / guard-only

why now:
  the scheduler boundary, marker, ledger record, ledger consume, and roundtrip
  surfaces are all implemented. A scalar-lane closeout prevents drift before
  the next selection row decides whether to keep advancing allocator behavior
  or reopen real scheduler substrate.

why not real scheduler substrate:
  worker handoff, run/progress semantics, and source concurrency remain broader
  substrate work. They should not be opened implicitly by the scalar closeout.

why not language/compiler sidecar:
  no current proof is blocked on source-level concurrency features or compiler
  acceptance.

stop lines:
  no allocator behavior
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
current blocker moves to MIMAP-077A.
```
