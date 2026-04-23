---
Status: Active
Date: 2026-04-23
Scope: MapBox bad-key normalization after source-route write-return parity.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-95-mapbox-hako-extended-route-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-99-mapbox-write-return-contract-card.md
---

# MapBox Bad-Key Contract Card

## Decision

MapBox source-visible methods that require a string key return the same stable
bad-key text when the key is not a string:

```text
[map/bad-key] key must be string
```

Covered methods:

| Method | Non-string key return |
| --- | --- |
| `set(key, value)` | `[map/bad-key] key must be string` |
| `get(key)` | `[map/bad-key] key must be string` |
| `has(key)` | `[map/bad-key] key must be string` |
| `delete(key)` | `[map/bad-key] key must be string` |
| `remove(key)` | same as `delete(key)` |

Field rows keep their separate field-name text:

```text
[map/bad-key] field name must be string
```

## Boundary

- Owner: `lang/src/runtime/collections/map_state_core_box.hako`
- Dispatch entry: `lang/src/vm/boxes/mir_vm_s0_boxcall_builtin.hako`
- Rust VM source-visible precedent:
  `src/backend/mir_interpreter/handlers/boxes_map.rs`
- Existing archived witnesses:
  `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/*_bad_key_ported_vm.sh`

## Out Of Scope

- `MapBox.get(missing-key)` text. It remains `[map/missing] Key not found: <key>`.
- `delete/remove(missing-key)` receipt. It remains `Key not found: <key>`.
- `keys()` / `values()` element publication.
- Rust object API internals that are not source-visible dispatch handlers.

## Implementation Gate

- `MapStateCoreBox.apply_has(...)` must publish the same handle/string bad-key
  return as `set/get/delete`, not return an unimplemented tag.
- Rust VM source-visible `MapBox.has(non-string)` must match the same bad-key
  text.
- Add a phase-291x vm-hako smoke that pins `set/get/has/delete/remove` bad-key
  lines without touching missing-key behavior.
- Add or reactivate a quick VM smoke for `MapBox.has(non-string)`.

## Next Slice

Implement the bad-key normalization, then move to `MapBox.get(missing-key)`
contract review or defer it behind element publication.
