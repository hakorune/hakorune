# 293x-649 MIMAP-141A Post-Guard-Spec-Pilot Row Selection

Status: landed
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

## Landed Result

`MIMAP-141A` selected `MIMAP-142A`.

Reason:

```text
The guard cleanup sidecar is closed through the first declarative-spec pilot.
The allocator lane can now return to the modeled local-free reuse ledger:
after MIMAP-138A applies a release to the source reuse ledger, the same modeled
reuse token should be recordable again as a new live row while live duplicates
remain rejected.
```

`MIMAP-141A` did not add allocator behavior, compiler route behavior, source
syntax, provider activation, backend matchers, or silent fallback.
