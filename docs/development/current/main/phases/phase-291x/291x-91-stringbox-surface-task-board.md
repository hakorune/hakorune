---
Status: Active
Date: 2026-04-22
Scope: phase-291x first implementation target, `StringBox` surface catalog。
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
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
- `apps/std/string2.hako` remains diagnostic residue in this card
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
- `std.string2.hako` cleanup
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
