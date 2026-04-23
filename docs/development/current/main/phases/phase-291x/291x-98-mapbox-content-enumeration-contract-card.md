---
Status: Landed
Date: 2026-04-23
Scope: MapBox `keys()` / `values()` content enumeration contract after source-route size parity.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-95-mapbox-hako-extended-route-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-102-mapbox-keys-values-element-publication-card.md
  - docs/development/current/main/phases/phase-291x/291x-124-mapbox-element-publication-deferred-closeout-card.md
---

# MapBox Content Enumeration Contract Card

## Decision

At this card's landing time, source-level vm-hako promoted `MapBox.keys()` and
`MapBox.values()` only as ArrayBox-like shape rows with correct `size()` parity.
That provisional element-enumeration deferral is now superseded by `291x-102`.

This is a provisional size-only contract, not a claim that element reads are
unsupported forever. It prevents `keys().get(i)` / `values().get(i)` from being
silently treated as valid before the ArrayBox-like publication path can preserve
element kind, handle text, ordering, and copy metadata.

## Closeout Note (2026-04-24)

This provisional size-only decision was superseded by `291x-102`.

Current landed contract:

- Rust `values()` follows the same sorted-key order as `keys()`.
- source-level vm-hako publishes `keys()/values()` element state through the S0
  state owner.
- `keys().get(i)` and `values().get(i)` are pinned by the 291x-102 acceptance
  smoke.

## Ordering Audit Note (verified 2026-04-23)

Current Rust source state in `src/boxes/map_box.rs`:

- `keys()` (lines 199-210): collects all keys then calls `.sort()` —
  deterministic lexical order is confirmed.
- `values()` (lines 212-224): iterates `self.data.read().unwrap().values()`
  directly without sorting — iteration order matches the underlying `HashMap`
  hash order, which is **NOT** the same as the sorted key order returned by
  `keys()`.

Therefore, the later element-enumeration promotion needed a Rust-side fix to
`values()` (sort by key before collecting) in addition to the source-level
vm-hako publication path. That fix is landed in `291x-102`; do not use this
audit note as the current Rust behavior.

## Superseded Future Contract

When content enumeration was promoted, the desired contract was:

- `keys()` returns keys in deterministic lexical key order.
- `values()` returns values in the same sorted key order as `keys()`, not hash
  storage iteration order.
- source-level vm-hako must publish ArrayBox-like element state through the
  same owner as `MapStateCoreBox`, not through a deleted bridge or runtime
  handle fallback.

## Historical Implementation Gate

Element enumeration was not allowed until all of these were true:

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

Write-return (`291x-99`), bad-key (`291x-100`), get-missing-key (`291x-101`),
and keys/values element publication (`291x-102`) contracts are all landed. Do
not reopen already-landed slices without an owner-path change.
