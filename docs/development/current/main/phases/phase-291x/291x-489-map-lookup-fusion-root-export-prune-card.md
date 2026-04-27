---
Status: Landed
Date: 2026-04-27
Scope: Prune MapLookupFusionRoute root export and route-local enum visibility
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-487-map-lookup-fusion-route-field-closeout-card.md
  - src/mir/mod.rs
  - src/mir/map_lookup_fusion_plan.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-489: MapLookupFusion Root Export Prune

## Goal

Keep MapLookup fusion metadata owned by `map_lookup_fusion_plan` instead of
letting route-local records leak through the root `crate::mir` surface.

This follows the prior field-boundary cleanup:

```text
map_lookup_fusion_plan
  owns route construction and route-local enum vocabulary

runner JSON emitter
  consumes MapLookupFusionRoute through stable accessors only
```

## Inventory

Current external uses:

- `src/mir/function/types.rs`
  - already imports `MapLookupFusionRoute` from
    `crate::mir::map_lookup_fusion_plan`
- `src/runner/mir_json_emit/root.rs`
  - still names `crate::mir::MapLookupFusionRoute`
- `src/mir/mod.rs`
  - still re-exports `MapLookupFusionRoute` plus route-local enums

No external consumer needs to construct or match:

- `MapLookupFusionOp`
- `MapLookupFusionProof`
- `MapLookupStoredValueProof`

## Boundary

- BoxShape-only.
- Do not change MapLookup fusion detection.
- Do not change route metadata JSON.
- Do not change route ids, proof strings, effect tags, or lowering tiers.
- Do not touch `.inc` metadata readers or helper symbols.

## Acceptance

- Root `crate::mir` no longer re-exports MapLookup fusion route-local types.
- Route-local enums are private to `map_lookup_fusion_plan`.
- JSON emitter names the owner module path or imports from owner module.
- `cargo check -q` passes.
- MapLookup fusion JSON and planner tests pass.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.

## Result

Landed.

- Removed MapLookup fusion route-local type exports from root `crate::mir`.
- Kept `MapLookupFusionRoute` available through
  `crate::mir::map_lookup_fusion_plan`.
- Made MapLookup fusion route-local enums private to their owner module.
- Updated the JSON emitter type annotation to use the owner module path.
- Preserved MapLookup fusion detection, metadata JSON, proof strings, effect
  tags, lowering tiers, `.inc` readers, and helper symbols.

Verification:

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

Note: quick gate still reports the existing chip8 release artifact sync
warning, then completes with `[dev-gate] profile=quick ok`.
