---
Status: Landed
Date: 2026-04-27
Scope: Close out MapLookupFusionRoute public field boundary cleanup lane
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-485-map-lookup-fusion-route-field-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-486-map-lookup-fusion-route-field-cleanup-card.md
  - src/mir/map_lookup_fusion_plan.rs
  - src/runner/mir_json_emit/root.rs
  - src/runner/mir_json_emit/tests/map_lookup_fusion_routes.rs
---

# 291x-487: MapLookupFusionRoute Field Closeout

## Result

The MapLookupFusionRoute public field boundary cleanup lane is closed.

- `MapLookupFusionRoute` fields are owner-private.
- JSON emission uses stable read accessors.
- Runner JSON tests consume owner-provided `#[cfg(test)]` fixture support.
- Runner JSON tests no longer import `MapLookupFusionRoute` construction
  records, route enums, generic-method facts, or `ValueId`.
- JSON field names, values, route id, effect tags, proof strings, and lowering
  tiers are unchanged.
- `.inc` readers and const-fold lowering behavior are unchanged.

## Verification

The implementation card verified:

```bash
cargo check -q
cargo test -q build_mir_json_root_emits_map_lookup_fusion_routes
cargo test -q map_lookup_fusion
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

Note: quick gate still reports the existing chip8 release artifact sync warning,
then completes with `[dev-gate] profile=quick ok`.

## Next

Select the next phase-291x compiler-cleanliness lane as a separate BoxShape
card. Do not mix this with `.inc` mirror pruning or hot lowering.
