# 212x-90 Placement / Effect Agg Local Fold SSOT

Status: SSOT

## Goal

- keep `placement_effect_routes` as the generic folded route inventory
- widen it with landed `agg_local` proof that already implies a stable placement/effect reading

## In Scope

- fold these `agg_local_scalarization_routes` into `placement_effect_routes`:
  - `sum_local_layout(...)`
  - `user_box_local_body(...)`
- keep these routes out of `placement_effect_routes`:
  - `typed_slot_storage(...)`

## Fixed Decisions

- this cut remains inspection-only
- `agg_local_scalarization_routes` remains the owner seam for the broader agg-local layer
- `placement_effect_routes` only reads the subset that already implies a stable local aggregate placement/effect decision
- storage-only routes remain agg-local-only because they are not placement/effect decisions

## Out of Scope

- changing refresh order
- changing backend consumers
- deleting family-specific metadata
- adding new placement/effect rewrite passes

## Acceptance

- placement/effect fold-up includes placement-relevant agg-local routes
- typed-slot storage is explicitly excluded from the fold-up
- MIR JSON export stays deterministic
