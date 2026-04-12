# 226x-90 Placement-Effect String Scheduling Owner Cut SSOT

Status: SSOT

## Decision

- `placement_effect_transform` is the optimizer-facing owner for module iteration and pre/post-DCE placement/effect scheduling
- `string_corridor_sink` remains the family-local owner for function-level string rewrites
- this cut is BoxShape only: no new acceptance shape, no new proof schema, no behavior widening

## Why

- `phase225x` created the top-level owner seam, but the landed string lane still exposed scheduling shape from the family module
- the next generic placement/effect cuts need one optimizer-facing owner that can schedule family transforms without reintroducing family-specific wiring at the pipeline boundary

## In Scope

- route pre/post-DCE scheduling through function-level string sink helpers
- keep folded metadata refresh inside the string family module
- lock focused unit tests for both pre-DCE and post-DCE entry points

## Out of Scope

- widening string proof vocabulary
- removing remaining compatibility fallbacks inside string family logic
- starting `semantic simplification bundle`

## Acceptance

1. `placement_effect_transform` owns module iteration and calls family-local helpers
2. `string_corridor_sink` no longer needs a non-test function-level scheduling wrapper for the optimizer boundary
3. focused placement/effect and string sink tests stay green
4. `tools/checks/dev_gate.sh quick` stays green
