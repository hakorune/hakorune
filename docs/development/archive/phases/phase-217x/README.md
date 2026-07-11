# Phase 217x: user-box micro seed thin-entry fold

Status: Landed

Purpose
- land the next `generic placement / effect` proving slice on the boundary pure-first user-box path
- make the current generic thin-entry helper read folded `placement_effect_routes` first

Scope
- switch `hako_llvmc_has_thin_entry_selection(...)` to `placement_effect_routes` first
- keep `thin_entry_selections` as compatibility fallback
- pin folded route fixtures for:
  - user-box field inline-scalar routes
  - user-box method known-receiver routes
- keep the cut behavior-preserving on the current boundary pure-first lane

Non-goals
- no deletion of `thin_entry_selections`
- no backend-wide generic transform
- no string seam widening
- no MIR lowering shape rewrite

Acceptance
- current boundary pure-first user-box consumers succeed from `placement_effect_routes` first
- legacy `thin_entry_selections` still works as fallback
- `git diff --check`

