# 293x-609 MIMAP-110A Post-Local-Free-Candidate-Ledger Row Selection

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-110A` is the planning row selected by `MIMAP-109A`.

The segment allocation modeled lane now has:

```text
modeled consume
  -> allocation ledger
  -> release span facts
  -> released-span ledger
  -> local-free candidate ledger
```

This row should select exactly one next allocator behavior, closeout,
substrate, or narrow Hakorune acceptance row using the mimalloc validation
cadence.

## Scope

- Review `MIMAP-109A` evidence and the current segment allocation modeled lane.
- Apply
  `docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md`.
- Select exactly one next row.

## Stop Lines

- No new `.hako` behavior.
- No cleanup bundle.
- No real segment allocation/free execution.
- No free-list mutation.
- No page state mutation.
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
| `110A.1` | Review current evidence after MIMAP-109A. | selected row cites validation level. | no behavior |
| `110A.2` | Pick exactly one next row. | new selected card exists. | no bundle |
| `110A.3` | Update current pointers. | current pointer guard passes. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
