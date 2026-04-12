# Phase 213x: sum outer-box sinking consumer seed

Status: Landed

Purpose
- land the first proving slice under `generic placement / effect`
- make the current sum lowering read the folded generic placement/effect inventory first for local aggregate routes

Scope
- seed sum local-aggregate resolver maps from `placement_effect_routes`
- keep current `sum_placement_selections` / `sum_placement_layouts` as fallback metadata
- keep lowering behavior-preserving

Follow-on
- broader `generic placement / effect` proving slices

Non-goals
- no canonical MIR rewrite
- no generic backend-wide consumer switch
- no removal of the sum-specific metadata lanes
- no widening beyond the current sum outer-box sinking route

Acceptance
- current sum lowering reads `placement_effect_routes` first for the local-aggregate route
- legacy sum placement metadata still works as fallback
- `git diff --check`
