# 293x-633 MIMAP-127A Post-Local-Free-Reuse Row Selection

Status: landed
Date: 2026-05-18

## Decision

`MIMAP-127A` is a planning-only row after `MIMAP-126A` proved modeled
local-free reuse through `HakoAllocPageModel.acquire(size)`.

The next step should select exactly one next mimalloc / hako_alloc allocator row
or a focused Hakorune compiler row if the next allocator behavior exposes a
real compiler acceptance blocker.

## Scope

- Review the segment allocation modeled lane through `MIMAP-126A`.
- Pick exactly one next row.
- Prefer allocator behavior progress unless the next behavior has a concrete
  compiler acceptance blocker.
- Keep the selection small enough for one owner, one proof app or guard, and one
  commit.

## Stop Lines

- No allocator behavior in this planning row.
- No compiler route behavior in this planning row.
- No source syntax change.
- No cleanup bundle.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No backend `.inc` matcher.
- No silent fallback.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Result

`MIMAP-127A` selected:

```text
MIMAP-128A
  segment allocation modeled local-free reuse closeout guard
```

Rationale:

```text
MIMAP-126A added a new allocator behavior owner and proof. The next durable
slice should freeze the owner/proof/guard/export/stop-line set before adding
the next allocator behavior.
```
