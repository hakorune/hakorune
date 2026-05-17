# 293x-568 MIMAP-081A Post-Segment-Arena-Bitmap-Inventory Row Selection

Status: landed
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

## Evidence Review

Landed evidence:

```text
MIMAP-079A: segment / arena / bitmap boundary inventory
MIMAP-080A: local-run closeout guard for that inventory
```

Longer-lived SSOT context:

```text
docs/development/current/main/design/mimalloc-lifecycle-rewrite-blueprint-ssot.md
docs/development/current/main/investigations/mimalloc-source-concept-inventory.md
docs/development/current/main/design/mimalloc-substrate-representation-gap-ledger-ssot.md
```

The source concept inventory names `mi_segment_t`, commit/purge masks, segment
abandon/reclaim, and arena/bitmap helpers. The gap ledger keeps raw pointer
residence, atomic bitmap execution, OSVM execution, and provider activation as
separate gaps. Therefore the next row should not open bitmap or OSVM execution
yet.

## Selection Result

Selected next row:

```text
MIMAP-082A segment lifecycle scalar state contract
```

Reason:

`MIMAP-082A` can move from inventory to a small allocator-owned scalar contract
for the segment lifecycle vocabulary already named by the lifecycle blueprint:

```text
Reserved -> Active
Active -> PurgeScheduled
PurgeScheduled -> Purged
Active -> Abandoned
Abandoned -> Reclaimed
Reclaimed -> Active
Active/Purged -> Freed
```

The row must remain scalar/proof-only. It must not add raw pointer residence,
atomic bitmap claim/unclaim, OSVM execution, real scheduling, source-level
concurrency, provider activation, host allocator replacement, or backend
matchers.

