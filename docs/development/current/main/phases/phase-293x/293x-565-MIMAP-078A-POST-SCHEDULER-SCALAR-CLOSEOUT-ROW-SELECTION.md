# 293x-565 MIMAP-078A Post-Scheduler-Scalar-Closeout Row Selection

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-078A` is the planning row selected by `MIMAP-077A`.

The scalar scheduler lane is now closed out. This row must choose exactly one
follow-up before broader allocator behavior, real scheduler substrate work, or
Hakorune language work continues.

## Candidate Rows

| Candidate | Shape | Notes |
| --- | --- | --- |
| `MIMAP-079A` | allocator inventory | name segment / arena / bitmap representation boundaries after scheduler scalar closeout |
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

## Evidence Review

The scheduler scalar lane is now closed through MIMAP-077A. The remaining
upstream mimalloc concepts that are not yet represented as current `.hako`
owners include segment ownership, arena backing, and bitmap/commit-mask
residence.

Those concepts are not ready for real OSVM, atomic bitmap, raw pointer, or
provider behavior. They can still move forward as an allocator-local scalar
inventory row that reports the exact boundary and fail-fast reasons.

No proof app currently exposes a source-language or compiler acceptance
blocker.

## Selection Result

`MIMAP-078A` selects `MIMAP-079A`.

```text
row:
  MIMAP-079A segment arena bitmap boundary inventory

classification:
  allocator inventory / scalar representation boundary

why now:
  after the reclaim scheduler scalar lane is closed, the next unmodeled
  mimalloc concept family is segment/arena/bitmap ownership. Naming that
  boundary first prevents future allocator rows from smuggling raw pointers,
  atomic bitmaps, OSVM execution, or provider activation into small proofs.

why not real scheduler substrate:
  worker handoff, run/progress semantics, and source concurrency remain broader
  substrate work.

why not direct bitmap execution:
  atomic bitmap claim and packed/raw residence are representation/substrate
  gaps. This row should inventory and fail-fast them, not implement them.

stop lines:
  no real thread scheduling
  no worker spawning
  no source-level concurrency feature change
  no raw pointer residence
  no atomic bitmap execution
  no page-source call
  no OSVM unreserve / release
  no provider activation
  no host allocator replacement
  no backend matcher
```

Closeout:

```text
current blocker moves to MIMAP-079A.
```
