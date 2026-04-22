---
Status: Active
Date: 2026-04-22
Scope: phase-291x first implementation target, `StringBox` surface catalog。
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
  - docs/development/current/main/phases/phase-291x/291x-96-corebox-router-unified-value-path-card.md
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
- do not solve `lastIndexOf(needle, start_pos)` in the same commit as cataloging
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
| `291x-S15` | parked | move the next CoreBox method family through the same route pattern |

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
- `lastIndexOf` one-arg only
- `contains`

## Explicitly Deferred

- `lastIndexOf(needle, start_pos)`
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

## Router Follow-up

- current router policy allowlists only these StringBox families to
  `Route::Unified`: `length` / `len` / `size`, `substring` / `substr`, and
  `concat`, `trim`, `contains`, one-arg `lastIndexOf`, `replace`, and
  `indexOf` / `find`
- `boxcall_emit.rs` still bridges `MirType::String` receivers to `StringBox`
  before route selection
- `ArrayBox.length` / `size` / `len` is the first collection route slice
- `ArrayBox.push` is the first collection write route slice
- `ArrayBox.slice` is the next read-only route slice with an explicit
  `ArrayBox` return annotation
- `MapBox.size` is the first MapBox route slice; `len` is still a separate
  current-vtable row and stays deferred
- `MapBox.len` is the second MapBox route slice; it stays a separate row from
  `size` and does not introduce a `length` alias
- next safe cleanup is not a whole-CoreBox flip; it should allowlist one
  proven CoreBox method family, with remaining ArrayBox rows and MapBox as
  separate candidates
- remaining router cleanup after MapBox len: ArrayBox `get` / `set` /
  `pop` / `remove` / `insert` and remaining MapBox rows
- two-arg `lastIndexOf(needle, start_pos)` remains deferred and must stay off
  the allowlist until a dedicated runtime card lands
- tracking card: `291x-96-corebox-router-unified-value-path-card.md`
