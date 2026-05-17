# 293x-568 MIMAP-081A Post-Segment-Arena-Bitmap-Inventory Row Selection

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-081A` is the planning row selected by `MIMAP-080A`.

The segment / arena / bitmap boundary inventory is now implemented and closed
behind local-run guards. This row should review the current mimalloc lane and
select exactly one next row without adding allocator behavior.

## Scope

- Review landed MIMAP rows through `MIMAP-080A`.
- Decide whether the next row is allocator behavior, allocator substrate,
  Hakorune language/compiler acceptance, or a cleanup/closeout sidecar.
- Keep mimalloc as a `.hako` / `hako_alloc` allocator completeness lane, not a
  default process allocator replacement lane.

## Stop Lines

- No new `.hako` behavior.
- No new guard beyond the selected next row's card.
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
| `081A.1` | Review current landed allocator rows. | row selection cites evidence through MIMAP-080A. | no behavior |
| `081A.2` | Pick one next row. | new card exists and is selected current. | no bundle |
| `081A.3` | Update current pointers. | current pointer guard passes. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

