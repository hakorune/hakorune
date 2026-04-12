# 213x-90 Sum Outer-Box Sinking Consumer Seed SSOT

Status: SSOT

## Goal

- prove the first generic placement/effect consumer slice without changing the lowering outcome
- move the current sum outer-box sinking route to `placement_effect_routes` first

## In Scope

- read `placement_effect_routes` for:
  - `sum_placement` local-aggregate selection on `variant_make`
  - `agg_local_scalarization` `sum_local_layout(...)`
- seed the existing sum lowering resolver maps from that folded inventory
- preserve `sum_placement_selections` / `sum_placement_layouts` as compatibility fallback

## Fixed Decisions

- this cut is consumer-seed only
- `sum_ops` keeps its current local aggregate runtime shape
- the folded generic inventory is primary; family-specific sum metadata is fallback
- `user_box_local_body(...)` and storage-only routes must not seed sum-local resolver maps

## Out of Scope

- deleting sum-specific metadata
- changing local sum aggregate IR shape
- changing escape/objectization barriers
- widening generic placement/effect to other families in the same cut

## Acceptance

- resolver seeding can succeed from `placement_effect_routes` alone for the current sum local-aggregate path
- old sum placement metadata still keeps current tests green
