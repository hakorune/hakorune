---
Status: Landed
Date: 2026-04-23
Scope: MapBox `keys()` / `values()` content enumeration contract after source-route size parity.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-95-mapbox-hako-extended-route-cleanup-card.md
---

# MapBox Content Enumeration Contract Card

## Decision

For source-level vm-hako in this phase, `MapBox.keys()` and `MapBox.values()`
are promoted only as ArrayBox-like shape rows with correct `size()` parity.
Element enumeration is explicitly deferred.

This is a provisional size-only contract, not a claim that element reads are
unsupported forever. It prevents `keys().get(i)` / `values().get(i)` from being
silently treated as valid before the ArrayBox-like publication path can preserve
element kind, handle text, ordering, and copy metadata.

## Required Future Contract

When content enumeration is promoted, use the same visible order as the Rust
`MapBox` surface:

- `keys()` returns keys in deterministic lexical key order.
- `values()` returns values in the same key order as `keys()`, not hash storage
  iteration order.
- source-level vm-hako must publish ArrayBox-like element state through the
  same owner as `MapStateCoreBox`, not through a deleted bridge or runtime
  handle fallback.

## Implementation Gate For Future Promotion

Do not implement element enumeration until all of these are true:

- `ArrayCoreBox.get` can read VM-local ArrayBox-like element metadata before
  attempting runtime-handle `get_i64`.
- string keys have an explicit handle/publication path, not scalar coercion.
- value elements preserve their current kind (`scalar`, `bool`, `handle`) across
  MIR `copy`.
- a smoke pins at least `keys().size()`, `keys().get(0)`, and a matching
  `values().get(0)` result for a two-entry map.

## Current Acceptance

Current source-level vm-hako acceptance remains:

- `MapBox.keys().size()` equals the map size.
- `MapBox.values().size()` equals the map size.
- `MapBox.clear()` resets both size and ArrayBox-like key shape length.

## Next Slice

Move next to MapBox write-return contract decision. Return normalization is
separate from content enumeration and should not be mixed with element
publication.
