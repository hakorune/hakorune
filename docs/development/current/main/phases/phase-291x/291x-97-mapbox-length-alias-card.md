---
Status: Landed
Date: 2026-04-23
Scope: MapBox `length` alias contract-first cleanup after phase-292x closeout.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md
  - docs/development/current/main/phases/phase-291x/291x-91-stringbox-surface-task-board.md
  - docs/development/current/main/phases/phase-291x/291x-93-mapbox-surface-task-board.md
  - docs/development/current/main/phases/phase-291x/291x-96-corebox-router-unified-value-path-card.md
---

# MapBox Length Alias Card

## Decision

Add `MapBox.length()` as a read-only catalog alias for the existing Map size
surface.

This slice is intentionally narrow:

- `length` has arity 0 and returns the same integer value as `size` / `len`.
- `length` is an alias resolved by `MapMethodId`; it does not add a new slot.
- `size` keeps slot `200`; `len` keeps slot `201`.
- `set` / `delete` / `clear` return contracts are unchanged.
- `keys` / `values` / `delete` / `remove` / `clear` source-route promotion stays
  in the extended-route cleanup card.

## Implementation Shape

```text
MapBox.length/0
  -> MapMethodId catalog alias
  -> MapBox::invoke_surface(...)
  -> MIR return-type annotation as Integer
  -> router Unified value path for arity 0 only
  -> phase291x MapBox surface smoke
```

## Acceptance

```bash
cargo test -q map_surface_catalog --lib
cargo test -q map_length_row_uses_unified_value_path --lib
bash tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_surface_catalog_vm.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landing Snapshot

- `MapMethodId::Size` now owns `length` as an alias.
- No new MapBox slot was added; `size` remains slot `200`, and `len` remains
  slot `201`.
- `MapBox.length/0` publishes `MirType::Integer` and routes through the Unified
  value path.
- The MapBox surface smoke now runs through the direct Rust VM catalog route so
  vm-hako subset BoxCall debt does not own this Rust surface contract.
