---
Status: Landed
Date: 2026-04-27
Scope: Inventory GenericMethodRoute evidence record split
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-455-next-lane-selection-card.md
  - src/mir/generic_method_route_plan.rs
  - src/mir/map_lookup_fusion_plan.rs
  - src/runner/mir_json_emit/root.rs
  - src/runner/mir_json_emit/tests/generic_method_routes.rs
---

# 291x-456: GenericMethodRoute Evidence Inventory

## Goal

Inventory the `GenericMethodRoute` evidence fields before splitting them into a
named record.

## Evidence Fields

These are observed route evidence, not raw surface and not decided lowering
metadata:

- `receiver_origin_box`
- `key_route`

## Consumers

- `src/mir/generic_method_route_plan.rs`
  - Route constructors set receiver origin and key route after MIR observation.
  - Tests assert the observed evidence for direct and RuntimeData surfaces.
- `src/mir/map_lookup_fusion_plan.rs`
  - Fusion reads `receiver_origin_box` and `key_route` from
    `GenericMethodRoute`.
  - This must remain a consumer of route evidence, not a re-deriver.
- `src/runner/mir_json_emit/root.rs`
  - JSON output emits the same public field names.
- `src/runner/mir_json_emit/tests/generic_method_routes.rs`
  - Hand-built route fixtures need the new evidence record.

Out of scope:

- `MapLookupFusionRoute.receiver_origin_box`
- `MapLookupFusionRoute.key_route`

Those are output metadata for the fusion route itself and should not be moved
in this lane.

## Planned Change

- Add `GenericMethodRouteEvidence`.
- Replace flat `receiver_origin_box` and `key_route` fields on
  `GenericMethodRoute` with `evidence`.
- Add thin accessors:
  - `receiver_origin_box() -> Option<&str>`
  - `key_route() -> Option<GenericMethodKeyRoute>`
- Preserve JSON field names and values.

## Acceptance

```bash
cargo test -q generic_method_route
cargo test -q build_mir_json_root_emits_generic_method_routes
cargo test -q map_lookup_fusion
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
