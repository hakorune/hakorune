# 293x-635 MIMAP-129A Post-Local-Free-Reuse-Closeout Row Selection

Status: landed
Date: 2026-05-18

## Decision

`MIMAP-129A` is a planning-only row after the modeled local-free reuse closeout.

The next step should select exactly one next mimalloc / hako_alloc allocator row
or a focused Hakorune compiler row if the next allocator behavior exposes a
real compiler acceptance blocker.

## Scope

- Review the segment allocation modeled lane through the reuse closeout.
- Pick exactly one next row.
- Prefer allocator behavior progress unless the next row exposes a concrete
  compiler acceptance blocker.
- Keep the selection small enough for one owner, proof app or guard, and one
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

`MIMAP-129A` selected:

```text
MIMAP-130A
  segment allocation modeled local-free reuse ledger route
```

Rationale:

```text
MIMAP-126A proves page-local reuse through HakoAllocPageModel.acquire(size),
but the reused block is not yet represented as a new live modeled allocation
row. The existing segment allocation modeled ledger is bump-shaped and requires
modeled_block_start == old_page_used, so reusing a local_free block should use a
dedicated reuse ledger instead of widening the bump ledger in place.
```
