# 293x-599 MIMAP-102A Post-Segment-Allocation-Modeled-Recycle Row Selection

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-102A` is the planning row selected by `MIMAP-101A`.

The segment allocation modeled ledger now records, releases, and recycles a
released scalar token without opening real segment execution. This row should
review the current modeled segment allocation lifecycle and select exactly one
next row without bundling allocator behavior.

## Scope

- Review landed MIMAP rows through `MIMAP-101A`.
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

## Selection Result

`MIMAP-102A` selects
`HAKO-ALLOC-SRC-CLEAN-001 segment counter compound assignment cleanup`.

Rationale:

- C199 compound assignment is already accepted for field targets and lowers to
  the same canonical assignment form.
- The segment allocation modeled lane now contains many same-field diagnostic
  counter increments such as `me.x = me.x + 1`.
- A focused cleanup row can improve `.hako` readability without changing
  allocator behavior or adding parser/compiler work.

The selected row is deliberately mechanical and narrow: only exact same-field
`me.FIELD = me.FIELD + 1` increments in the current segment allocation memory
owners may become `me.FIELD += 1`.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `102A.1` | Review current landed allocator rows. | row selection cites evidence through MIMAP-101A. | no behavior |
| `102A.2` | Pick one next row. | new card exists and is selected current. | no bundle |
| `102A.3` | Update current pointers. | current pointer guard passes. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
```
