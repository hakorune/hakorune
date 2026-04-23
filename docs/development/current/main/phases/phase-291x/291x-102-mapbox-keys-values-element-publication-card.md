---
Status: Open
Date: 2026-04-23
Scope: MapBox keys()/values() element publication — promotion from size-only to element-readable.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-98-mapbox-content-enumeration-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-101-mapbox-get-missing-key-contract-card.md
---

# MapBox keys()/values() Element Publication Card

## Purpose

This card scopes the promotion of `MapBox.keys()` and `MapBox.values()` from
their current size-only contract (landed in `291x-98`) to element-readable
ArrayBox-like state.  It does not implement anything; it records the required
preconditions, the Rust-side fix, and the desired contract so the work can start
cleanly when the gates are met.

## Ordering Audit (from 291x-98, not to be restated)

Rust source state as of 2026-04-23 (`src/boxes/map_box.rs`):

- `keys()` (lines 199-210): already calls `.sort()` → deterministic lexical order.
- `values()` (lines 212-224): iterates `self.data.read().unwrap().values()`
  without sorting → order follows `HashMap` hash order, **not** key order.

Pairing `keys().get(i)` with `values().get(i)` is therefore incorrect with
current Rust code.  A Rust-side fix to `values()` is a hard prerequisite for
this card.

## Required Rust Fix (must land before or with element promotion)

In `src/boxes/map_box.rs` `values()` implementation:

- Collect keys in sorted order.
- Return values in the same sorted-key order, not in `HashMap` iteration order.
- This makes `values().get(i)` correspond to `keys().get(i)` for any valid `i`.

No other Rust-side behavior changes are in scope for this fix.

## Desired Contract (after promotion)

| Method | Order | Notes |
| --- | --- | --- |
| `keys()` | deterministic lexical key order | already true; do not regress |
| `values()` | same order as `keys()` — sorted by key | requires Rust fix above |

Element reads:

- `keys().get(i)` → the `i`-th key string for a zero-based index within
  `[0, keys().size())`.
- `values().get(i)` → the stored value for the `i`-th key in sorted key order.
- Out-of-range `get(i)` follows the existing `ArrayCoreBox.get` out-of-range
  contract (not re-decided here).

Publication path:

- source-level vm-hako publishes element state through the same owner as
  `MapStateCoreBox`, not through a deleted bridge or a new runtime-handle
  fallback.
- `ArrayCoreBox.get` reads VM-local ArrayBox-like element metadata before
  attempting a runtime-handle `get_i64`.

## Implementation Gates (from 291x-98; all must be true before promotion)

1. `ArrayCoreBox.get` can read VM-local ArrayBox-like element metadata before
   attempting runtime-handle `get_i64`.
2. String keys have an explicit handle/publication path, not scalar coercion.
3. Value elements preserve their current kind (`scalar`, `bool`, `handle`)
   across MIR `copy`.
4. A smoke pins at least `keys().size()`, `keys().get(0)`, and a matching
   `values().get(0)` result for a two-entry map.

## Out Of Scope

- `MapBox.get(existing-key)` successful-read type narrowing (data-dependent;
  separate card).
- `forEach`, `toJSON`, `setField`, `getField`.
- `MapBox.size` / `len` slot unification.
- `compat-only` exports in `crates/nyash_kernel/src/plugin/map_compat.rs`.

## Boundary

- Rust fix owner: `src/boxes/map_box.rs` (`values()` implementation)
- Source-level vm-hako owner: `lang/src/runtime/collections/map_state_core_box.hako`
- ArrayCoreBox gate owner: `lang/src/runtime/collections/array_core_box.hako`

## Acceptance Smoke (required when this card is promoted)

A phase-291x smoke must pin:

```
keys().size()       // two-entry map
keys().get(0)       // first key in lexical order
keys().get(1)       // second key in lexical order
values().get(0)     // value for first key
values().get(1)     // value for second key
```

All five assertions must hold in one smoke run without tolerance for hash-order
non-determinism.

## Next Slice

No further MapBox work is scoped after this card in phase-291x.
Once this card is promoted, phase-291x MapBox work is complete.
