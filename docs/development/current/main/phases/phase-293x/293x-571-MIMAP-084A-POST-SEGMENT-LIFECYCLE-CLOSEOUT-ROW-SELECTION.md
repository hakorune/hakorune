# 293x-571 MIMAP-084A Post-Segment-Lifecycle-Closeout Row Selection

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-084A` is the planning row selected by `MIMAP-083A`.

The segment lifecycle scalar state contract is now implemented and closed
behind local-run guards. This row should review the current mimalloc lane and
select exactly one next row without adding allocator behavior.

## Scope

- Review landed MIMAP rows through `MIMAP-083A`.
- Decide whether the next row is allocator behavior, allocator substrate,
  Hakorune language/compiler acceptance, or a cleanup/closeout sidecar.
- Keep mimalloc as a `.hako` / `hako_alloc` allocator completeness lane, not a
  default process allocator replacement lane.

## Stop Lines

- No new `.hako` behavior.
- No new guard beyond the selected next row's card.
- No segment allocation/free execution.
- No arena backing allocation.
- No segment map pointer membership.
- No real thread scheduling.
- No worker spawning.
- No source-level concurrency feature change.
- No raw pointer residence.
- No atomic bitmap execution.
- No page-source call.
- No OSVM unreserve / release.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.
- No backend `.inc` app/name matcher.
- No cleanup bundle.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `084A.1` | Review current landed allocator rows. | row selection cites evidence through MIMAP-083A. | no behavior |
| `084A.2` | Pick one next row. | new card exists and is selected current. | no bundle |
| `084A.3` | Update current pointers. | current pointer guard passes. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

