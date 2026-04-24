---
Status: Landed reference
Date: 2026-04-24
Scope: phase-291x first implementation target, `StringBox` surface catalog。
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
  - docs/development/current/main/phases/phase-291x/291x-96-corebox-router-unified-value-path-card.md
  - docs/development/current/main/phases/phase-291x/291x-103-stringbox-lastindexof-start-card.md
  - docs/development/current/main/phases/phase-291x/291x-104-mapbox-delete-remove-router-card.md
  - docs/development/current/main/phases/phase-291x/291x-105-mapbox-clear-router-card.md
  - docs/development/current/main/phases/phase-291x/291x-112-arraybox-clear-router-card.md
  - docs/development/current/main/phases/phase-291x/291x-113-arraybox-contains-router-card.md
  - docs/development/current/main/phases/phase-291x/291x-114-arraybox-indexof-router-card.md
  - docs/development/current/main/phases/phase-291x/291x-115-arraybox-join-router-card.md
  - docs/development/current/main/phases/phase-291x/291x-116-arraybox-reverse-router-card.md
  - docs/development/current/main/phases/phase-291x/291x-117-arraybox-sort-router-card.md
  - docs/development/current/main/phases/phase-291x/291x-118-arraybox-slice-result-receiver-card.md
  - docs/development/current/main/phases/phase-291x/291x-119-docs-status-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-123-stringbox-taskboard-router-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-125-stringbox-startswith-router-card.md
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
| `291x-S27` | done | prove `MapBox.clear` through the same MIR router Unified value path |
| `291x-S28` | done | land alias SSOT cleanup for manifest alias vs imported static-box binding vs type-name lowering |
| `291x-S29` | done | land conservative `MapBox.get(existing-key)` value publication for literal-key homogeneous receivers |
| `291x-S30` | done | land StringBox case-conversion catalog promotion for `toUpper` / `toLower` plus compatibility aliases |
| `291x-S31` | done | promote `ArrayBox.clear` as a catalog-backed receiver-only write-`Void` Unified row |
| `291x-S32` | done | promote `ArrayBox.contains` as a catalog-backed receiver-plus-value read-`Bool` Unified row |
| `291x-S33` | done | promote `ArrayBox.indexOf` as a catalog-backed receiver-plus-value read-`Integer` Unified row |
| `291x-S34` | done | promote `ArrayBox.join` as a catalog-backed receiver-plus-delimiter read-`String` Unified row |
| `291x-S35` | done | promote `ArrayBox.reverse` as a catalog-backed receiver-only write-`String` Unified row |
| `291x-S36` | done | promote `ArrayBox.sort` as a catalog-backed receiver-only write-`String` Unified row |
| `291x-S37` | done | pin `ArrayBox.slice()` result follow-up calls on the `ArrayBox` receiver path |
| `291x-S38` | done | promote `StringBox.startsWith(prefix)` as a catalog-backed receiver-plus-prefix read-`Bool` Unified row |

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
- `toUpper` with `toUpperCase` compatibility alias
- `toLower` with `toLowerCase` compatibility alias
- `lastIndexOf` one-arg and two-arg
- `contains`
- `startsWith`

## Explicitly Deferred StringBox Rows

- `split`
- `endsWith`
- `charAt`
- `equals` method surface

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
- `291x-108` landed the alias SSOT cleanup: manifest alias lookup stays in
  `hako.toml`, imported static-box alias binding stays in runner text merge, and
  MIR static-call lowering consumes only the explicit alias -> box binding.
- `291x-111` landed StringBox case conversion as stable surface rows:
  `toUpper` / `toLower` now come from the catalog, and `toUpperCase` /
  `toLowerCase` stay pinned as compatibility aliases.
- `291x-112` landed `ArrayBox.clear()` as the next stable Array write row:
  catalog-backed, receiver-only, and `Void` on the Unified value path.
- `291x-113` landed `ArrayBox.contains(value)` as the next stable Array read
  row: catalog-backed, receiver-plus-value, and `Bool` on the Unified value
  path.
- `291x-114` landed `ArrayBox.indexOf(value)` as the next stable Array read
  row: catalog-backed, receiver-plus-value, and `Integer` on the Unified value
  path.
- `291x-115` landed `ArrayBox.join(delimiter)` as the next stable Array read
  row: catalog-backed, receiver-plus-delimiter, and `String` on the Unified
  value path.
- `291x-116` landed `ArrayBox.reverse()` as the next stable Array mutating
  row: catalog-backed, receiver-only, and receipt `String` on the Unified value
  path.
- `291x-117` landed `ArrayBox.sort()` as the final deferred Array order row:
  catalog-backed, receiver-only, and receipt `String` on the Unified value
  path.
- `291x-118` landed the `ArrayBox.slice()` result-receiver pin: direct source
  `slice().length()` stays on the `ArrayBox` receiver path and does not lower
  as `RuntimeDataBox.length`.
- `291x-125` landed `StringBox.startsWith(prefix)` as the next stable
  StringBox read row: catalog-backed, receiver-plus-prefix, and `Bool` on the
  Unified value path.

## Router Follow-up

- current router policy allowlists only these StringBox families to
  `Route::Unified`: `length` / `len` / `size`, `substring` / `substr`, and
  `concat`, `trim`, `toUpper` / `toUpperCase`, `toLower` / `toLowerCase`,
  `contains`, `startsWith`, one-arg and two-arg `lastIndexOf`, `replace`, and
  `indexOf` / `find`
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
- `ArrayBox.clear` follows the same write-`Void` contract as `push` / `set` /
  `insert`, with a receiver-only Unified shape
- `ArrayBox.contains` follows the read-only Bool-return contract, with a
  receiver-plus-value Unified shape
- `ArrayBox.indexOf` follows the read-only Integer-return contract, with a
  receiver-plus-value Unified shape
- `ArrayBox.join` follows the read-only String-return contract, with a
  receiver-plus-delimiter Unified shape
- `ArrayBox.reverse` follows the mutating String-receipt contract, with a
  receiver-only Unified shape
- `ArrayBox.sort` follows the same mutating String-receipt contract, with a
  receiver-only Unified shape
- `ArrayBox.slice()` result follow-up calls stay on the `ArrayBox` receiver
  path; direct source `slice().length()` must not lower as `RuntimeDataBox.length`
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
  result type initially stayed `Unknown`; `291x-S29` now publishes `V` only for
  receiver-local homogeneous Map facts with tracked literal keys and keeps
  `Unknown` for mixed, untyped, and missing-key reads
- `MapBox.set` is the first stored-value MapBox write route slice; its
  visible write-return is the landed receipt `String`
- `MapBox.delete` / `remove` is the first mutating delete row route slice; its
  visible write-return is the landed receipt `String`
- `MapBox.clear` is the landed final mutating reset row route slice; its visible
  write-return is the landed receipt `String`
- `MapBox.length` is landed as a read-only alias slice; it maps to the
  existing Map size surface without unifying the `size` and `len` slots
- current stable route-only CoreBox rows are closed for ArrayBox stable rows and
  MapBox `size/length/len/has/get/set/keys/values/delete/remove/clear`
- future route work must be opened as a new one-family card; do not do a
  whole-CoreBox flip
- two-arg `lastIndexOf(needle, start_pos)` is landed in the `291x-103`
  runtime card and enters the allowlist through its catalog row
- tracking card: `291x-96-corebox-router-unified-value-path-card.md`
