# Phase 214x: user-box local body consumer seed

Status: Landed

Purpose
- land the second proving slice under `generic placement / effect`
- make the current user-box local aggregate seeding read the folded generic placement/effect inventory first

Scope
- seed user-box local body subject checks from folded `placement_effect_routes`
- keep current thin-entry subject lookup as compatibility fallback
- keep lowering behavior-preserving

Follow-on
- broader `generic placement / effect` proving slices

Non-goals
- no canonical MIR rewrite
- no generic backend-wide consumer switch
- no deletion of the thin-entry metadata lanes
- no widening beyond the current user-box local aggregate route

Acceptance
- current user-box local aggregate seeding reads folded `placement_effect_routes` first for selected local-body subjects
- legacy thin-entry subject lookup still works as fallback
- `git diff --check`
