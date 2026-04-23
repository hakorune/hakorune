---
Status: Active
Date: 2026-04-22
Scope: phase-291x first implementation target, `StringBox` surface catalog。
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
  - docs/development/current/main/phases/phase-291x/291x-96-corebox-router-unified-value-path-card.md
  - docs/development/current/main/phases/phase-291x/291x-103-stringbox-lastindexof-start-card.md
  - docs/development/current/main/phases/phase-291x/291x-104-mapbox-delete-remove-router-card.md
---

# StringBox Surface Task Board

## North Star

```text
StringBox surface catalog
  -> one method id table
  -> one invoke seam
  -> thin consumers
  -> one stable smoke
```

## Rules

- code slice is StringBox-only
- `apps/std/string.hako` stays sugar
- legacy `apps/std/string2.hako` cleanup is handled by a separate cleanup card
- `lastIndexOf(needle, start_pos)` must stay isolated in `291x-103`
- do not widen replacement semantics; record current behavior and keep the card small

## Docs Slice

| Card | Status | Goal |
| --- | --- | --- |
| `291x-0a` | done | create phase front + design brief + StringBox taskboard + inventory ledger |
| `291x-0b` | done | update current/restart/workstream pointers to phase-291x |
| `291x-0c` | done | lock first StringBox stable surface row set |

## Implementation Slice Order

| Card | Status | Goal |
| --- | --- | --- |
| `291x-S1a` | done | add `src/boxes/basic/string_surface_catalog.rs` and `StringMethodId` |
| `291x-S1b` | done | add `StringBox::invoke_surface(...)` for the first stable rows |
| `291x-S1c` | done | convert TypeRegistry / method resolution / effect analysis readers |
| `291x-S1d` | done | route Rust VM slot dispatch and `.hako` VM-facing `StringCoreBox` through the cataloged surface rows where receiver is `StringBox`/`String` |
| `291x-S1e` | done | add stable StringBox surface smoke |
| `291x-S2` | done | prove `StringBox.length` / `len` / `size` through the MIR router Unified value path |
| `291x-S3` | done | prove `StringBox.substring` / `substr` through the same MIR router Unified value path |
| `291x-S4` | done | prove `StringBox.concat` through the same MIR router Unified value path |
| `291x-S5` | done | prove `StringBox.trim` through the same MIR router Unified value path |
| `291x-S6` | done | prove `StringBox.contains` through the same MIR router Unified value path with Bool return publication |
| `291x-S7` | done | prove one-arg `StringBox.lastIndexOf` through the same MIR router Unified value path |
| `291x-S8` | done | prove `StringBox.replace` through the same MIR router Unified value path |
| `291x-S9` | done | prove `StringBox.indexOf` / `find` one-arg and two-arg through the same MIR router Unified value path |
| `291x-S10` | done | prove `ArrayBox.length` / `size` / `len` through the same MIR router Unified value path |
| `291x-S11` | done | prove `ArrayBox.push` through the same MIR router Unified value path |
| `291x-S12` | done | prove `ArrayBox.slice` through the same MIR router Unified value path |
| `291x-S13` | done | prove `MapBox.size` through the same MIR router Unified value path |
| `291x-S14` | done | prove `MapBox.len` through the same MIR router Unified value path |
| `291x-S15` | done | prove `MapBox.has` through the same MIR router Unified value path |
| `291x-S16` | done | prove `ArrayBox.get` through the same MIR router Unified value path |
| `291x-S17` | done | prove `ArrayBox.pop` through the same MIR router Unified value path |
| `291x-S18` | done | prove `ArrayBox.set` through the same MIR router Unified value path |
| `291x-S19` | done | prove `ArrayBox.remove` through the same MIR router Unified value path |
| `291x-S20` | done | prove `ArrayBox.insert` through the same MIR router Unified value path |
| `291x-S21` | done | prove `MapBox.get` through the same MIR router Unified value path |
| `291x-S22` | done | prove `MapBox.set` through the same MIR router Unified value path |
| `291x-S23` | done | add `MapBox.length` as the first contract-first MapBox cleanup row |
| `291x-S24` | done | decide and land the MapBox extended owner path for `keys` / `values` / `delete` / `remove` / `clear` |
| `291x-S25` | done | prove two-arg `StringBox.lastIndexOf(needle, start_pos)` through the catalog and Unified value path |
| `291x-S26` | done | prove `MapBox.delete` / `remove` through the same MIR router Unified value path |

## First Stable Surface Target

- `length`
- `len` alias
- `size` alias
- `substring`
- `substr` alias
- `concat`
- `indexOf`
- `find` alias
- `replace`
- `trim`
- `lastIndexOf` one-arg and two-arg
- `contains`

## Explicitly Deferred

- `split`
- `startsWith`
- `endsWith`
- uppercase/lowercase name family
- `charAt`
- `equals` method surface
- MapBox implementation

## Exit Condition

This StringBox slice is done when:

1. catalog is the clear Rust-side surface authoring point
2. TypeRegistry aliases come from the catalog
3. dispatch no longer hardcodes the first stable StringBox slot rows in several places
4. `.hako` VM-facing `StringCoreBox` remains a thin consumer for the same stable rows, including boolean kind publication for `contains`
5. one stable smoke proves aliases and output values without VM stub drift

## Landing Snapshot

- Rust catalog: `src/boxes/basic/string_surface_catalog.rs`
- Rust invoke seam: `StringBox::invoke_surface(...)`
- Rust consumers: TypeRegistry, method resolution, effect analysis, and VM slot dispatch
- `.hako` consumer: `lang/src/runtime/collections/string_core_box.hako`
- smoke: `tools/smokes/v2/profiles/integration/apps/phase291x_stringbox_surface_catalog_vm.sh`

## Cleanup Snapshot

- legacy diagnostic stub `apps/std/string2.hako` was deleted after the catalog
  landed; it was not a full surface owner and had no active import route.
- `StringBox.lastIndexOf(needle, start_pos)` is landed through the catalog and
  Unified value path; the dedicated smoke pins the optional start-position
  behavior.

## Router Follow-up

- current router policy allowlists only these StringBox families to
  `Route::Unified`: `length` / `len` / `size`, `substring` / `substr`, and
  `concat`, `trim`, `contains`, one-arg and two-arg `lastIndexOf`, `replace`,
  and `indexOf` / `find`
- `boxcall_emit.rs` still bridges `MirType::String` receivers to `StringBox`
  before route selection
- `ArrayBox.length` / `size` / `len` is the first collection route slice
- `ArrayBox.push` is the first collection write route slice
- `ArrayBox.slice` is the next read-only route slice with an explicit
  `ArrayBox` return annotation
- `ArrayBox.get` is the first generic element-return route slice; its MIR
  result type intentionally stays `Unknown` because the element type is
  data-dependent
- `ArrayBox.pop` follows the same generic element-return contract as `get`;
  its MIR result type intentionally stays `Unknown`
- `ArrayBox.set` follows the same write-`Void` contract as `push`, with a
  receiver-plus-index-plus-value Unified shape
- `ArrayBox.remove` follows the same generic element-return contract as
  `get` / `pop`; its MIR result type intentionally stays `Unknown`
- `ArrayBox.insert` follows the same write-`Void` contract as `push` / `set`,
  with a receiver-plus-index-plus-value Unified shape
- `MapBox.size` is the first MapBox route slice; `len` was kept as a separate
  current-vtable row and handled in its own slice
- `MapBox.len` is the second MapBox route slice; it stays a separate row from
  `size` and does not introduce a `length` alias
- `MapBox.has` is the first keyed MapBox read route slice and publishes a
  fixed `Bool` result
- `MapBox.get` is the first stored-value MapBox read route slice; its MIR
  result type intentionally stays `Unknown`
- `MapBox.set` is the first stored-value MapBox write route slice; its
  visible write-return is the landed receipt `String`
- `MapBox.delete` / `remove` is the first mutating delete row route slice; its
  visible write-return is the landed receipt `String`
- `MapBox.length` is landed as a read-only alias slice; it maps to the
  existing Map size surface without unifying the `size` and `len` slots
- next safe cleanup is not a whole-CoreBox flip; it should allowlist one
  proven CoreBox method family at a time
- remaining route-only CoreBox rows are closed for ArrayBox stable rows and
  MapBox `size/length/len/has/get/set/keys/values/delete/remove`; remaining
  mutating MapBox row is `clear`
- MapBox `clear` stays contract-first until its router promotion is pinned
- two-arg `lastIndexOf(needle, start_pos)` is landed in the `291x-103`
  runtime card and enters the allowlist through its catalog row
- tracking card: `291x-96-corebox-router-unified-value-path-card.md`
