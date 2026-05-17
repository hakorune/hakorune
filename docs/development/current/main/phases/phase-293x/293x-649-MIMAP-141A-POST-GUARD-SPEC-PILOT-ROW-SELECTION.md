# 293x-649 MIMAP-141A Post-Guard-Spec-Pilot Row Selection

Status: selected current
Date: 2026-05-18

## Decision

`MIMAP-141A` is a planning-only row after the guard manifest declarative spec
pilot. It should select exactly one next mimalloc / hako_alloc allocator row, or
one focused Hakorune compiler row if the next allocator behavior exposes a real
acceptance blocker.

## Scope

- Review the segment allocation modeled lane after the guard cleanup sidecar.
- Pick exactly one next row.
- Prefer returning to allocator behavior progress unless a concrete compiler
  acceptance blocker is already visible.

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
