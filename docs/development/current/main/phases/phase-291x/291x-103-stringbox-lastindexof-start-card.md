---
Status: Landed implementation card
Date: 2026-04-23
Scope: Promote `StringBox.lastIndexOf(needle, start_pos)` into the phase-291x catalog-backed value path.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md
  - docs/development/current/main/phases/phase-291x/291x-91-stringbox-surface-task-board.md
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
  - docs/development/current/main/phases/phase-291x/291x-96-corebox-router-unified-value-path-card.md
---

# StringBox lastIndexOf Start-Position Card

## Goal

Close the deferred StringBox surface row:

```text
StringBox.lastIndexOf(needle, start_pos) -> Integer
```

This is a StringBox-only cleanup card. It must not widen unrelated string
families, MapBox rows, std sugar, or `.inc` codegen behavior.

## Contract

- canonical method: `lastIndexOf`
- aliases: none
- arity: `2`
- slot: `308`
- effect: `Read`
- return: `Value` / Integer
- start position is clamped into the receiver length range
- matching searches backward for the last occurrence whose start index is at
  or before `start_pos`
- one-arg `lastIndexOf(needle)` remains unchanged and searches the full string

Examples:

```text
"banana".lastIndexOf("na")     -> 4
"banana".lastIndexOf("na", 3)  -> 2
"banana".lastIndexOf("na", 1)  -> -1
```

## Required Edges

- Rust catalog row in `src/boxes/basic/string_surface_catalog.rs`
- Rust helper support in `src/boxes/string_ops.rs`
- Rust VM fallback handlers accept one or two args consistently
- `.hako` VM-facing `StringCoreBox` delegates to the `.hako` search kernel
- `StringSearchKernelBox` owns the reverse-search policy
- router allowlist promotes arity-2 only through the catalog row
- type annotation returns `MirType::Integer` through the catalog row
- phase-291x StringBox smoke pins one-arg and two-arg output

## Non-Goals

- `split`
- `startsWith` / `endsWith`
- uppercase/lowercase method family
- `charAt`
- `equals`
- std module import-policy cleanup
- `.inc` codegen thinning

## Landing Snapshot

- Rust catalog row: `StringMethodId::LastIndexOfFrom`
- Rust helper: `string_ops::last_index_of_from(...)`
- Rust VM boundary: one-arg and two-arg `lastIndexOf` share the same helper
- `.hako` owner: `StringSearchKernelBox.last_index_from(...)`
- VM source boundary: duplicate receiver normalization stays at the method
  boundary before invoking the catalog-backed surface
- smoke: `tools/smokes/v2/profiles/integration/apps/phase291x_stringbox_lastindexof_start_vm.sh`

## Acceptance

```bash
cargo test string_surface_catalog --lib
cargo test router --lib
cargo test corebox_surface_aliases_use_catalog_return_type --lib
bash tools/smokes/v2/profiles/integration/apps/phase291x_stringbox_surface_catalog_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase291x_stringbox_lastindexof_start_vm.sh
```

The final commit should also keep the current pointer guard green.
