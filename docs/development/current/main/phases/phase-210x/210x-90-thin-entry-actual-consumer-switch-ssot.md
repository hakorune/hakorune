# 210x-90 Thin-Entry Actual Consumer Switch SSOT

Status: SSOT

## Goal

- fold the scattered thin-entry lookup logic into one shared consumer seam for the lowering side
- keep `thin_entry_candidates` / `thin_entry_selections` as the metadata source, but stop duplicating the lookup and decision policy in each instruction handler

## In Scope

- user-box method known-receiver route selection
- user-box field get/set inline-scalar route selection
- user-box local-body inline-scalar route selection
- shared Python helper for thin-entry selection lookup

## Pilot Boundary Notes

- thin-entry metadata stays in MIR JSON / resolver seed
- lowering remains the actual consumer, not a new semantic owner
- current behavior must remain unchanged while the consumer seam is centralized

## Fixed Decisions

- the current `user_box_method.known_receiver` selector is the first actual-consumer slice
- field access and user-box local-body consumers should reuse the same helper seam
- closure call / entry ABI switching stays for a later cut

## Out of Scope

- closure call widening
- generic placement/effect
- agg_local scalarization
- DCE / simplification bundle

## Acceptance

- method / field / user-box-local lowering share one thin-entry decision helper
- the current thin-entry consumer tests stay green
- docs point at `phase210x` as the current code phase
