---
Status: Landed
Date: 2026-04-27
Scope: Make MapLookupFusionRoute field layout owner-private and update JSON consumers
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-485-map-lookup-fusion-route-field-inventory-card.md
  - src/mir/map_lookup_fusion_plan.rs
  - src/runner/mir_json_emit/root.rs
  - src/runner/mir_json_emit/tests/map_lookup_fusion_routes.rs
---

# 291x-486: MapLookupFusionRoute Field Cleanup

## Goal

Make `MapLookupFusionRoute` field layout MIR-owned while preserving the JSON
metadata contract.

## Result

- Made `MapLookupFusionRoute` fields private.
- Added stable read accessors for JSON emission.
- Added owner-provided `#[cfg(test)]` fixture support for the JSON test.
- Replaced runner JSON direct field reads/construction with accessors/fixture.

## Boundaries

- BoxShape-only.
- Preserve JSON field names, values, route ids, effect tags, proof strings, and
  lowering tiers.
- Do not change MapLookup fusion detection, refresh ordering, `.inc` readers,
  or const-fold lowering.

## Verification

Ran:

```bash
cargo check -q
cargo test -q build_mir_json_root_emits_map_lookup_fusion_routes
cargo test -q map_lookup_fusion
```
