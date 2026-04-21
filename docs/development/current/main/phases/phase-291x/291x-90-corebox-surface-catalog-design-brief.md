---
Status: Active
Date: 2026-04-22
Scope: CoreBox surface catalog の横断 vocabulary / first rows / implementation boundaries。
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-91-stringbox-surface-task-board.md
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
  - docs/development/current/main/phases/phase-290x/README.md
---

# CoreBox Surface Catalog Design Brief

## Problem

CoreBox methods are still duplicated across:

- runtime implementation
- VM slot dispatch
- TypeRegistry exposure
- method resolution / effect analysis
- `.hako` core wrappers
- std module sugar
- smokes and docs

phase-290x fixed the first ArrayBox seam. phase-291x turns that pattern into a
repeatable CoreBox surface rule.

## Catalog Row Shape

Each stable surface row must carry:

| Field | Meaning |
| --- | --- |
| `id` | typed method id used by dispatch / invoke |
| `canonical` | one user-facing name that docs prefer |
| `aliases` | compatibility names that route to the same row |
| `arity` | exact argument count for this row |
| `slot` | TypeRegistry / vtable slot |
| `effect` | `Read` or `WriteHeap` for current consumers |
| `return` | `Value` or `Void` |
| `exposure` | runtime / VM / std / smoke pin state |

## Current CoreBox Snapshot

| Box | State | First catalog owner |
| --- | --- | --- |
| `ArrayBox` | landed in phase-290x | `src/boxes/array/surface_catalog.rs` |
| `StringBox` | active first implementation | planned `src/boxes/basic/string_surface_catalog.rs` |
| `MapBox` | inventory only in this slice | planned later, after StringBox |

## StringBox First Stable Rows

The first StringBox implementation slice should only catalog methods already
expected by current runtime/docs/smoke paths.

| Canonical | Aliases | Arity | Slot | Effect | Return | Notes |
| --- | --- | ---: | ---: | --- | --- | --- |
| `length` | `len`, `size` | 0 | 300 | Read | Value | canonical is `length`; aliases are compatibility |
| `substring` | `substr` | 2 | 301 | Read | Value | one-arg substring remains a separate compatibility topic |
| `concat` |  | 1 | 302 | Read | Value | pure string construction |
| `indexOf` | `find` | 1 | 303 | Read | Value | `find` is compatibility alias |
| `indexOf` | `find` | 2 | 303 | Read | Value | second arg is start position |
| `replace` |  | 2 | 304 | Read | Value | existing primitive/StringBox routes differ on all-vs-first replacement; do not widen this card |
| `trim` |  | 0 | 305 | Read | Value | existing route |
| `lastIndexOf` |  | 1 | 308 | Read | Value | two-arg form is deferred |
| `contains` |  | 1 | 309 | Read | Value | returns boolean |

Explicitly not in the first stable row set:

- `lastIndexOf(needle, start_pos)`
- `split`
- `startsWith` / `endsWith`
- `toUpper` / `toLower` / `toUpperCase` / `toLowerCase`
- `charAt`
- `equals`

These require separate exposure decisions because docs, Rust helpers, TypeRegistry,
and VM dispatch do not currently agree on them.

## MapBox Inventory Rows

MapBox should follow the same row shape, but it is not implemented in the first
StringBox card.

Known rows to inventory before coding:

| Canonical | Aliases | Expected slot family |
| --- | --- | --- |
| `length` | `len`, `size` | 200 / 201 currently split |
| `has` |  | 202 |
| `get` |  | 203 |
| `set` |  | 204 |
| `delete` | `remove` | 205 |
| `keys` |  | 206 |
| `values` |  | 207 |
| `clear` |  | 208 |

Decision needed before MapBox coding:

- whether `length`, `len`, and `size` collapse to one canonical slot row or keep
  legacy slot distinction for ABI compatibility.

## Guardrails

- Do not mix `StringBox` and `MapBox` implementation in one commit.
- Do not change language semantics while cataloging.
- Do not add hidden env toggles.
- No fallback dispatch for unknown methods. Unknown surface remains fail-fast.

