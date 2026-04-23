---
Status: Landed
Date: 2026-04-23
Scope: MapBox get missing-key contract after write-return and bad-key normalization.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-95-mapbox-hako-extended-route-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-100-mapbox-bad-key-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-102-mapbox-keys-values-element-publication-card.md
  - docs/development/current/main/phases/phase-291x/291x-124-mapbox-element-publication-deferred-closeout-card.md
---

# MapBox Get Missing-Key Contract Card

## Decision

`MapBox.get(key)` keeps the existing stable missing-key text across the Rust
visible route and the source-level vm-hako S0 owner:

```text
[map/missing] Key not found: <key>
```

This slice fixes the contract, not the successful-read value shape. A missing
read stays a tagged string witness rather than falling back to a stale scalar
`0`, and it remains distinct from:

- bad-key input: `[map/bad-key] key must be string`
- delete/remove missing-key receipt: `Key not found: <key>`

## Boundary

- Owner: `lang/src/runtime/collections/map_state_core_box.hako`
- Dispatch entry: `lang/src/vm/boxes/mir_vm_s0_boxcall_builtin.hako`
- Rust visible precedent: `src/boxes/map_box.rs`
- Stable diagnostics note: `lang/src/vm/README.md`

## Out Of Scope

- `MapBox.get(existing-key)` element publication and type narrowing
- `keys()` / `values()` element publication
- write-return receipt rows (`set/delete/remove/clear`)
- bad-key normalization
- `MapBox.get` MIR return-type widening; successful reads remain data-dependent

## Implementation Gate

- keep the existing visible string contract; do not introduce a new sentinel or
  fallback scalar for a missing read
- keep the owner on the existing vm-hako S0 route; no new by-name adapter
- pin the source-level vm-hako witness with a phase-291x smoke
- keep the quick core witness for the stable `[map/missing]` tag

## Landed Slice

- `MapBox.get(missing-key)` continues to publish
  `[map/missing] Key not found: <key>` instead of a scalar fallback.
- phase-291x source-level vm-hako witness:
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_get_missing_vm.sh`
- quick core witness:
  `tools/smokes/v2/profiles/quick/core/map/map_missing_key_tag_vm.sh`
- legacy capability witness remains valid:
  `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_get_missing_ported_vm.sh`

## Next Slice

`keys()/values()` element publication is landed in `291x-102`. Do not reopen
this missing-key contract unless the owner path changes.
