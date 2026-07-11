# 214x-90 User-Box Local Body Consumer Seed SSOT

Status: SSOT

## Goal

- prove the next generic placement/effect consumer slice without changing the lowering outcome
- move the current user-box local aggregate seed to folded `placement_effect_routes` first

## In Scope

- read `placement_effect_routes` for folded `agg_local_scalarization` `user_box_local_body(...)` rows
- distinguish field-set vs field-get subject proof using the folded row shape
- preserve thin-entry subject lookup as compatibility fallback

## Fixed Decisions

- this cut is consumer-seed only
- `user_box_local` keeps its current aggregate-local runtime shape
- the folded generic inventory is primary; thin-entry subject lookup is fallback
- sum layout rows and storage-only routes must not seed user-box local aggregate layouts

## Out of Scope

- deleting thin-entry metadata
- changing local user-box aggregate IR shape
- changing escape/materialization barriers
- widening generic placement/effect to other families in the same cut

## Acceptance

- resolver seeding can succeed from `placement_effect_routes` alone for the current user-box local aggregate route
- folded field-get rows must not silently stand in for folded field-set proof
- old thin-entry subject lookup still keeps current tests green
