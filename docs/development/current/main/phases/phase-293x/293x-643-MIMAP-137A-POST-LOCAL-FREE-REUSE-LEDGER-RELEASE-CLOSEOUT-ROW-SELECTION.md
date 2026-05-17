# 293x-643 MIMAP-137A Post-Local-Free-Reuse-Ledger-Release-Closeout Row Selection

Status: selected current
Date: 2026-05-18

## Decision

`MIMAP-137A` is a planning-only row after the modeled local-free reuse ledger
release closeout.

The next step should select exactly one next mimalloc / hako_alloc allocator row
or a focused Hakorune compiler row if the next allocator behavior exposes a
real compiler acceptance blocker.

## Scope

- Review the segment allocation modeled lane through the local-free reuse ledger
  release closeout.
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
