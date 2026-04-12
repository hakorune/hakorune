# 211x-90 Generic Placement / Effect Owner Seam SSOT

Status: SSOT

## Goal

- add the first MIR-side owner seam for the generic placement/effect layer
- keep the current family-specific pilots as proof sources, not competing semantic owners

## In Scope

- fold these landed pilots into one route inventory:
  - `string_corridor_candidates`
  - `sum_placement_selections`
  - `thin_entry_selections`
- define a generic folded route surface for:
  - borrowed/publication/materialization/direct-kernel decisions from string
  - local-aggregate/compat-runtime decisions from sum
  - public-entry/thin-internal-entry decisions from thin-entry
- export the folded routes through MIR JSON

## Fixed Decisions

- the folded route inventory is inspection-only in this cut
- string / sum / thin-entry remain the family-specific proof sources
- `placement_effect_routes` is the generic reader seam, not a rewrite pass
- semantic refresh owns the refresh order for the folded routes

## Out of Scope

- changing canonical MIR
- changing lowering behavior
- deleting the family-specific pilot metadata
- backend-side consumer widening

## Acceptance

- MIR owns `placement_effect_routes`
- semantic refresh populates the folded routes
- MIR JSON exports the folded routes
- `git diff --check`
