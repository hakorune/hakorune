---
Status: Landed
Date: 2026-04-27
Scope: Inventory GenericMethodRoute site/operands record split
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-459-next-lane-selection-card.md
  - src/mir/generic_method_route_plan.rs
  - src/mir/map_lookup_fusion_plan.rs
  - src/runner/mir_json_emit/root.rs
  - src/runner/mir_json_emit/tests/generic_method_routes.rs
---

# 291x-460: GenericMethodRoute Site/Operands Inventory

## Goal

Inventory the remaining flat `GenericMethodRoute` location and operand fields
before splitting them into named records.

## Site Fields

- `block`
- `instruction_index`

These identify the MIR call site that produced the route.

## Operand Fields

- `receiver_value`
- `key_value`
- `result_value`

These identify MIR values flowing through the call site. They are not surface
compatibility, observed route evidence, or lowering decision metadata.

## Consumers

- `src/mir/generic_method_route_plan.rs`
  - Route constructors set site and operands.
  - Route sorting reads `block` / `instruction_index`.
  - Scalar MapGet proof checks read site location.
  - Tests assert site and operand values.
- `src/mir/map_lookup_fusion_plan.rs`
  - Fusion pairs MapGet/MapHas by site order and consumes operands for
    receiver/key/result values.
  - This must remain a consumer of route site/operands, not a re-deriver.
- `src/runner/mir_json_emit/root.rs`
  - JSON output emits the same public field names.
- `src/runner/mir_json_emit/tests/generic_method_routes.rs`
  - Hand-built route fixtures need the new site and operands records.

Out of scope:

- Other route types in `root.rs`.
- `MapLookupFusionRoute` fields. Those are output metadata for the fusion route
  itself and should not be moved in this lane.

## Planned Change

- Add `GenericMethodRouteSite`.
- Add `GenericMethodRouteOperands`.
- Replace flat site and operand fields on `GenericMethodRoute` with `site` and
  `operands`.
- Add thin accessors:
  - `block() -> BasicBlockId`
  - `instruction_index() -> usize`
  - `receiver_value() -> ValueId`
  - `key_value() -> Option<ValueId>`
  - `result_value() -> Option<ValueId>`
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
