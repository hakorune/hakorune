# 293x-615 MIMAP-116A Post-Local-Free-Page-Apply Row Selection

Status: landed
Date: 2026-05-18

## Decision

`MIMAP-116A` is the planning row selected by `MIMAP-115A`.

The segment allocation modeled lane can now apply a successful scalar
local-free plan to an explicit `HakoAllocPageModel` through:

```text
HakoAllocPageModel.releaseLocal(block_id)
```

This row should select exactly one next allocator behavior, closeout,
substrate, or narrow Hakorune acceptance row using the mimalloc validation
cadence.

## Result

`MIMAP-116A` selects:

```text
MIMAP-117A segment allocation modeled local-free page-apply closeout guard
```

Validation cadence:

```text
L4 closeout row:
  manifest-backed row guard via run_row_guard.sh --only <row-id>
  public k2_wide wrapper
```

The selected row should freeze the page-model apply seam before any later
segment-map, raw pointer, arena, atomic bitmap, or provider-facing row is
selected.

## Scope

- Review `MIMAP-115A` evidence and the current page-model apply seam.
- Apply
  `docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md`.
- Select exactly one next row.

## Stop Lines

- No new `.hako` behavior.
- No cleanup bundle.
- No raw pointer residence.
- No segment-map pointer membership or lookup.
- No arena backing allocation.
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
| `116A.1` | Review current evidence after MIMAP-115A. | selected row cites validation level. | no behavior |
| `116A.2` | Pick exactly one next row. | new selected card exists. | no bundle |
| `116A.3` | Update current pointers. | current pointer guard passes. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
