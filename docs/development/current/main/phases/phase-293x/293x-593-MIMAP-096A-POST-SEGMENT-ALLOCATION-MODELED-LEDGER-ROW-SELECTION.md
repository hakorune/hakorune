# 293x-593 MIMAP-096A Post-Segment-Allocation-Modeled-Ledger Row Selection

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-096A` is the planning row selected by `MIMAP-095A`.

The segment allocation modeled ledger route is implemented and closed behind
guards. This row should review the current segment allocation model and select
exactly one next row without bundling allocator behavior.

## Scope

- Review landed MIMAP rows through `MIMAP-095A`.
- Decide whether the next row is another modeled segment allocation behavior,
  allocator substrate, Hakorune language/compiler acceptance, or a cleanup
  sidecar.
- Keep mimalloc as a `.hako` / `hako_alloc` allocator completeness lane, not a
  default process allocator replacement lane.

## Stop Lines

- No new `.hako` behavior.
- No new guard beyond the selected next row's card.
- No real segment allocation/free execution.
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
| `096A.1` | Review current landed allocator rows. | row selection cites evidence through MIMAP-095A. | no behavior |
| `096A.2` | Pick one next row. | new card exists and is selected current. | no bundle |
| `096A.3` | Update current pointers. | current pointer guard passes. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
