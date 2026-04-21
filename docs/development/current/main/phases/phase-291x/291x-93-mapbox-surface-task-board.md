---
Status: Landed
Date: 2026-04-22
Scope: phase-291x second implementation target, `MapBox` surface catalog first slice.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
---

# MapBox Surface Task Board

## North Star

```text
MapBox surface catalog
  -> one method id table for current vtable rows
  -> one Rust invoke seam
  -> thin consumers
  -> one stable smoke
```

## Rules

- code slice is MapBox-only
- catalog current Rust vtable rows first
- do not add new aliases in this card
- do not collapse `size` slot `200` and `len` slot `201`
- do not change `set` / `delete` / `clear` return values
- do not touch compat ABI exports except to document their current separation
- keep `.hako` `MapCoreBox` as the visible owner for state/raw-handle orchestration

## Implementation Slice Order

| Card | Status | Goal |
| --- | --- | --- |
| `291x-M1a` | done | add `src/boxes/map_surface_catalog.rs` and `MapMethodId` |
| `291x-M1b` | done | add `MapBox::invoke_surface(...)` for current vtable rows |
| `291x-M1c` | done | convert TypeRegistry / method resolution / effect analysis readers |
| `291x-M1d` | done | route Rust VM slot dispatch through the cataloged surface rows |
| `291x-M1e` | done | add stable MapBox surface smoke |

## First Stable Surface Target

| Method | Arity | Slot | Effect | Return | Notes |
| --- | ---: | ---: | --- | --- | --- |
| `size` | 0 | 200 | Read | Value | keep legacy slot |
| `len` | 0 | 201 | Read | Value | keep legacy slot |
| `has` | 1 | 202 | Read | Value | boolean value |
| `get` | 1 | 203 | Read | Value | missing-key behavior unchanged |
| `set` | 2 | 204 | WriteHeap | Value | current receipt value unchanged |
| `delete` / `remove` | 1 | 205 | WriteHeap | Value | preserve existing TypeRegistry alias |
| `keys` | 0 | 206 | Read | Value | array value |
| `values` | 0 | 207 | Read | Value | array value |
| `clear` | 0 | 208 | WriteHeap | Value | current receipt value unchanged |

## Explicitly Deferred

- `length` alias
- `size` / `len` slot unification
- `getField` / `setField` policy
- `birth`
- `forEach`
- `toJSON`
- compat-only exports in `crates/nyash_kernel/src/plugin/map_compat.rs`
- bad-key validation unification across all runtime lanes

## Exit Condition

This MapBox slice is done when:

1. catalog is the clear Rust-side surface authoring point
2. TypeRegistry rows come from the catalog instead of a separate Map extras table
3. method resolution and effect analysis read `MapMethodId`
4. Rust VM slot dispatch delegates current MapBox rows to `MapBox::invoke_surface(...)`
5. one stable smoke proves Rust catalog rows plus the hako-visible VM subset without stub drift

## Landing Snapshot

- Rust catalog: `src/boxes/map_surface_catalog.rs`
- Rust invoke seam: `MapBox::invoke_surface(...)`
- Rust consumers: TypeRegistry, method resolution, effect analysis, and VM slot dispatch
- removed duplicate TypeRegistry `MAP_METHOD_EXTRAS` table
- smoke: `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_surface_catalog_vm.sh`

## Follow-Up Cleanup Candidates

- `.hako` VM source route still stubs `keys` / `values` / `remove` / `clear`; promote only with a separate state-owner card.
- legacy `apps/std/map_std.hako` JIT-only placeholder was deleted after inventory; do not recreate it as a second std owner.
- unused `lang/src/vm/hakorune-vm/map_keys_values_bridge.hako` prototype was deleted; do not recreate it outside the active VM route owner.
- `apps/lib/boxes/map_std.hako` remains a P0 scaffold and is imported by `apps/selfhost-runtime/selfhost_prelude.hako`; delete only after replacing that prelude/module-registry dependency.
- `crates/nyash_kernel/src/plugin/map_compat.rs` remains compat-only quarantine; do not delete in a surface catalog commit.
