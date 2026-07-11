# 215x-90 Thin-Entry Consumer Seed SSOT

Status: SSOT

## Goal

- prove the next generic placement/effect consumer slice without changing the lowering outcome
- move the current thin-entry consumer seed to folded `placement_effect_routes` first

## In Scope

- read `placement_effect_routes` for folded `thin_entry` rows
- reconstruct the current thin-entry resolver row shape from folded route data
- preserve `thin_entry_selections` as compatibility fallback

## Fixed Decisions

- this cut is consumer-seed only
- current field/method lowering keeps the existing thin-entry helper seam
- the folded generic inventory is primary; family-specific thin-entry metadata is fallback
- non-thin-entry placement/effect rows must not seed thin-entry resolver maps

## Out of Scope

- deleting thin-entry metadata
- changing thin-entry lowering names or IR shape
- changing known-receiver / inline-scalar policy
- widening generic placement/effect to other families in the same cut

## Acceptance

- resolver seeding can succeed from `placement_effect_routes` alone for the current thin-entry consumer routes
- old thin-entry metadata still keeps current tests green
