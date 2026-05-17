# 293x-574 MIMAP-087A Post-Segment-Page-Membership-Closeout Row Selection

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-087A` is the planning row selected by `MIMAP-086A` and resumed after
the `GUARD-MANIFEST-001` through `GUARD-MANIFEST-010` cleanup burst.

The segment page membership scalar contract is now implemented and closed
behind local-run guards. This row should review the current mimalloc lane and
select exactly one next row without adding allocator behavior.

Selected next row:

```text
MIMAP-088A segment allocation readiness scalar contract
```

The next row stays proof-only and scalar. It composes the landed segment
lifecycle / page membership vocabulary into an allocation-readiness contract
without executing segment allocation/free, arena backing, raw pointer
residence, segment-map lookup, atomic bitmap claims, OSVM calls, thread
scheduling, provider activation, or backend matchers.

## Scope

- Review landed MIMAP rows through `MIMAP-086A`.
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
| `087A.1` | Review current landed allocator rows. | row selection cites evidence through MIMAP-086A. | no behavior |
| `087A.2` | Pick one next row. | `MIMAP-088A` card exists and is selected current. | no bundle |
| `087A.3` | Update current pointers. | current pointer guard passes. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
