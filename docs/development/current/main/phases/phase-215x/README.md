# Phase 215x: thin-entry consumer seed

Status: Landed

Purpose
- land the third proving slice under `generic placement / effect`
- make the current thin-entry consumer seed read the folded generic placement/effect inventory first

Scope
- seed current thin-entry resolver rows from folded `placement_effect_routes`
- keep current `thin_entry_selections` metadata as compatibility fallback
- keep lowering behavior-preserving

Follow-on
- broader `generic placement / effect` proving slices

Non-goals
- no canonical MIR rewrite
- no generic backend-wide consumer switch
- no deletion of the thin-entry metadata lanes
- no widening beyond the current thin-entry consumer seed

Acceptance
- current thin-entry consumer seeding reads folded `placement_effect_routes` first
- legacy `thin_entry_selections` still works as fallback
- `git diff --check`
