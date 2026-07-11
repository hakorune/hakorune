# 293x-603 MIMAP-105A Post-Release-Span-Facts Row Selection

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-105A` is the planning row selected by `MIMAP-104A`.

The scalar segment allocation ledger now records, releases, recycles, and
reports release span facts without opening real segment execution. This row
should select exactly one next mimalloc / hako_alloc row.

## Selection Result

`MIMAP-105A` selects
`MIMAP-ROW-CADENCE-001 mimalloc row validation cadence SSOT`.

Rationale:

- Recent rows are correct but heavy because each allocator slice carries docs,
  proof apps, MIR JSON checks, pure-first EXE checks, stop-line guards, and
  current pointer updates.
- The next productive step is to make the validation levels explicit so future
  rows do not guess whether to run a full guard, compatibility guards, or only
  current pointer checks.
- This is a process/SSOT cleanup row. It must not reduce the required evidence
  for already-landed allocator behavior rows.

## Scope

- Review the segment allocation modeled lane through `MIMAP-104A`.
- Decide whether the next row continues scalar segment allocation/free facts,
  returns to allocator substrate, or selects a narrow Hakorune acceptance
  sidecar.
- Keep mimalloc as a `.hako` / `hako_alloc` allocator completeness lane, not a
  default process allocator replacement lane.

## Stop Lines

- No new `.hako` behavior.
- No cleanup bundle.
- No real segment allocation/free execution.
- No free-list mutation.
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
| `105A.1` | Review the landed segment allocation modeled and span facts rows. | row selection cites the latest landed evidence. | no behavior |
| `105A.2` | Pick one next row. | new card exists and is selected current. | no bundle |
| `105A.3` | Update current pointers. | current pointer guard passes. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
```
