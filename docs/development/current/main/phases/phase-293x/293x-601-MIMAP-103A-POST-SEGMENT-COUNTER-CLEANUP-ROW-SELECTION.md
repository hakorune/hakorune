# 293x-601 MIMAP-103A Post-Segment-Counter-Cleanup Row Selection

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-103A` is the planning row selected by
`HAKO-ALLOC-SRC-CLEAN-001`.

The current segment allocation modeled lane is cleaner after the focused
counter compound-assignment rewrite. This row should select exactly one next
mimalloc / hako_alloc row without bundling allocator behavior.

## Selection Result

`MIMAP-103A` selects
`MIMAP-104A segment allocation modeled ledger release span facts route`.

Rationale:

- The modeled segment allocation ledger can record, release, and recycle scalar
  allocation tokens.
- The next allocator-relevant fact is the released block span: start, block
  count, end, allocation-time page usage, and remaining blocks.
- This keeps the row scalar and ledger-local while preparing future free-list
  / segment-free work without opening real free execution.

## Scope

- Review the segment allocation modeled lane through
  `HAKO-ALLOC-SRC-CLEAN-001`.
- Decide whether the next row should continue modeled segment allocation,
  return to allocator substrate, or select a narrow Hakorune acceptance sidecar.
- Keep mimalloc as a `.hako` / `hako_alloc` allocator completeness lane, not a
  default process allocator replacement lane.

## Stop Lines

- No new `.hako` behavior.
- No cleanup bundle.
- No real segment allocation/free execution.
- No arena backing allocation.
- No raw pointer residence.
- No segment-map pointer membership or lookup.
- No atomic bitmap execution.
- No page-source or OSVM execution.
- No real thread scheduling.
- No worker spawning.
- No source-level concurrency feature change.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.
- No backend `.inc` app/name matcher.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `103A.1` | Review the landed segment allocation modeled and cleanup rows. | row selection cites the latest landed evidence. | no behavior |
| `103A.2` | Pick one next row. | new card exists and is selected current. | no bundle |
| `103A.3` | Update current pointers. | current pointer guard passes. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
```
