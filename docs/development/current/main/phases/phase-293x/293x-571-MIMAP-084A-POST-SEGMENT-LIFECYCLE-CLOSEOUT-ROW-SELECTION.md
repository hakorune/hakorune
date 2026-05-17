# 293x-571 MIMAP-084A Post-Segment-Lifecycle-Closeout Row Selection

Status: landed
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

## Evidence Review

Landed evidence:

```text
MIMAP-079A: segment / arena / bitmap boundary inventory
MIMAP-080A: segment / arena / bitmap inventory closeout
MIMAP-082A: segment lifecycle scalar state contract
MIMAP-083A: segment lifecycle scalar state closeout
```

Longer-lived SSOT context:

```text
docs/development/current/main/design/mimalloc-lifecycle-rewrite-blueprint-ssot.md
docs/development/current/main/investigations/mimalloc-source-concept-inventory.md
lang/src/hako_alloc/memory/page_box.hako
```

The current allocator lane has page-local state and segment lifecycle state, but
it still lacks a scalar boundary that says a page/slice belongs to a segment
without using raw pointer membership or segment-map lookup.

## Selection Result

Selected next row:

```text
MIMAP-085A segment page membership scalar contract
```

Reason:

`MIMAP-085A` can connect the existing page model vocabulary to segment
lifecycle vocabulary using scalar identifiers only:

```text
segment_id
page_id
slice_index
segment_state
page_used
page_capacity
```

The row must remain scalar/proof-only. It must not add raw pointer residence,
segment-map pointer membership, arena backing allocation, atomic bitmap
claim/unclaim, OSVM execution, real scheduling, source-level concurrency,
provider activation, host allocator replacement, or backend matchers.

