---
Status: Landed
Date: 2026-04-23
Scope: MapBox write-return contract after source-route state parity.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-95-mapbox-hako-extended-route-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-98-mapbox-content-enumeration-contract-card.md
---

# MapBox Write Return Contract Card

## Decision

MapBox write rows return stable receipt strings across the Rust catalog route
and the source-level vm-hako S0 state owner.

Canonical source-level vm-hako success returns:

| Method | Return |
| --- | --- |
| `set(key, value)` | `Set key: <key>` |
| `delete(key)` | `Deleted key: <key>` |
| `remove(key)` | same as `delete(key)` |
| `clear()` | `Map cleared` |

Missing-key `delete` / `remove` preserves the existing visible receipt:

```text
Key not found: <key>
```

This contract intentionally removes the accidental source-level behavior where
`delete` returned the removed stored value. Write methods report the write
outcome; stored value publication belongs to `get` and to a future
`keys()/values()` element-publication contract.

## Boundary

- Owner: `lang/src/runtime/collections/map_state_core_box.hako`
- Dispatch entry: `lang/src/vm/boxes/mir_vm_s0_boxcall_builtin.hako`
- Source rewrite entry: `src/runner/reference/vm_hako/payload_normalize.rs`
- Rust precedent: `src/boxes/map_box.rs` and
  `src/boxes/map_surface_catalog.rs`

## Out Of Scope

- Bad-key normalization. Existing `[map/bad-key] ...` behavior stays as-is in
  this card.
- `get` missing-key normalization.
- `keys()` / `values()` element enumeration.
- `setField` / `getField` surface cleanup.
- `MapBox.size` / `len` slot unification.

## Implementation Gate

The implementation may change only the write-return publication shape and the
matching type hints/tests:

- `MapStateCoreBox.apply_set` writes a handle/string receipt for `set`.
- `MapStateCoreBox.apply_delete` writes a handle/string receipt for both the
  deleted and missing-key cases.
- `MapStateCoreBox.apply_clear` writes a handle/string receipt for `clear`.
- source-level vm-hako smoke pins the four visible lines:
  `Set key: a`, `Deleted key: a`, `Key not found: z`, and `Map cleared`.
- bad-key smokes must remain unchanged.

## Landed Slice

- `MapStateCoreBox.apply_set(...)` publishes `Set key: <key>` for `MapBox.set`.
- `MapStateCoreBox.apply_delete(...)` publishes `Deleted key: <key>` for
  existing keys and `Key not found: <key>` for missing keys.
- `MapStateCoreBox.apply_clear(...)` publishes `Map cleared`.
- `CoreMethodId::MapSet` and the MIR return-type hint paths now report
  `StringBox` / `MirType::String` for the write receipt contract.
- Smoke:
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_write_return_vm.sh`

## Next Slice

Move to the separate MapBox bad-key normalization decision.
