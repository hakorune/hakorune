# 293x-587 MIMAP-090A Post-Segment-Allocation-Readiness Row Selection

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-090A` is the planning row selected by `MIMAP-089A`.

The segment allocation readiness scalar contract is implemented and closed
behind local-run guards. This row should review the current segment / arena /
bitmap / allocation execution boundary and select exactly one next row without
bundling allocator behavior.

## Scope

- Review landed MIMAP rows through `MIMAP-089A`.
- Decide whether the next row is allocator behavior, allocator substrate,
  Hakorune language/compiler acceptance, or a cleanup/closeout sidecar.
- Keep mimalloc as a `.hako` / `hako_alloc` allocator completeness lane, not a
  default process allocator replacement lane.

## Stop Lines

- No new `.hako` behavior.
- No new guard beyond the selected next row's card.
- No segment allocation/free execution.
- No arena backing allocation.
- No raw pointer residence.
- No segment-map pointer membership or lookup.
- No atomic bitmap claim/unclaim.
- No page-source call.
- No OSVM execution, unreserve, or release.
- No real thread scheduling.
- No worker spawning.
- No source-level concurrency feature change.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.
- No backend `.inc` app/name matcher.
- No cleanup bundle.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `090A.1` | Review current landed allocator rows. | row selection cites evidence through MIMAP-089A. | no behavior |
| `090A.2` | Pick one next row. | new card exists and is selected current. | no bundle |
| `090A.3` | Update current pointers. | current pointer guard passes. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
