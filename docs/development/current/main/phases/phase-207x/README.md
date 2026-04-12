# Phase 207x: generic placement / effect docs-facts phase

Status: Landed

Purpose
- inventory the current pilot scaffolds under the generic placement / effect layer
- fix the first docs/facts boundary before any code widening

Scope
- align current pointers with the roadmap SSOT
- inventory the current pilot surfaces:
  - `string_corridor_facts`
  - `string_corridor_candidates`
  - `metadata.string_kernel_plans`
  - `sum_placement_facts`
  - `sum_placement_selections`
  - `sum_placement_layouts`
  - `thin_entry_candidates`
  - `thin_entry_selections`
- define the first owner boundary for the generic placement / effect layer
- keep family-specific pilot scaffolds as pilot scaffolds, not new top-level rows

Inventory Summary
- string corridor: facts -> candidates -> plan metadata
- sum placement: facts -> selections -> layouts
- thin-entry: candidates -> selections
- generic placement / effect: placement, publish, materialize, and direct-kernel legality

Non-goals
- no code changes
- no semantic widening
- no DCE widening
- no string / sum / user-box / array / map semantic change

Acceptance
- docs-only
- `git diff --check`
- current pointers point at this phase as the next docs/facts cut

Result
- `generic placement / effect` now has a dedicated docs/facts entry phase
- the next follow-on after this docs cut is `phase208x agg_local scalarization docs/facts phase`
