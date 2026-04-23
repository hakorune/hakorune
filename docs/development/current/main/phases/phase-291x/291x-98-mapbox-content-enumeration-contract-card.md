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

## Ordering Audit Note (verified 2026-04-23)

Current Rust source state in `src/boxes/map_box.rs`:

- `keys()` (lines 199-210): collects all keys then calls `.sort()` —
  deterministic lexical order is confirmed.
- `values()` (lines 212-224): iterates `self.data.read().unwrap().values()`
  directly without sorting — iteration order matches the underlying `HashMap`
  hash order, which is **NOT** the same as the sorted key order returned by
  `keys()`.

Therefore, any future element enumeration that pairs `keys().get(i)` with
`values().get(i)` requires a Rust-side fix to `values()` (sort by key before
collecting) in addition to the source-level vm-hako publication path.
Do not claim that current Rust `values()` already returns in key order.

## Required Future Contract

When content enumeration is promoted, the desired contract is:

- `keys()` returns keys in deterministic lexical key order (already true in
  Rust; confirmed by audit).
- `values()` **must be fixed** to return values in the same sorted key order as
  `keys()`, not the current hash storage iteration order.  This is a desired
  future state, not a description of current Rust behaviour.
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

Write-return (`291x-99`), bad-key (`291x-100`), and get-missing-key (`291x-101`)
contracts are all landed. Keep `keys()/values()` element publication deferred
until a dedicated follow-up card; do not reopen already-landed slices.
