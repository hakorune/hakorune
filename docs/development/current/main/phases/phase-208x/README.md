# Phase 208x: agg_local scalarization docs-facts phase

Status: Landed

Purpose
- inventory the current pilot scaffolds under the agg_local scalarization layer
- fix the first docs/facts boundary before any code widening

Scope
- align current pointers with the roadmap SSOT
- inventory the current pilot surfaces:
  - `sum_placement_facts`
  - `sum_placement_selections`
  - `sum_placement_layouts`
  - `thin_entry_candidates`
  - `thin_entry_selections`
  - selected user-box local bodies carried via `field_decls + thin_entry_selections.inline_scalar`
  - `ArrayBox` typed-slot pilot
  - tuple multi-payload compat transport as boundary-only
- define the first owner boundary for agg_local scalarization
- keep family-specific pilot scaffolds as pilots, not new top-level rows

Inventory Summary
- sum placement: facts -> selections -> layouts
- thin-entry: candidates -> selections
- user-box local bodies: `field_decls + thin_entry_selections.inline_scalar` -> MIR / MIR JSON / Program JSON local-body proof
- ArrayBox typed-slot: narrow typed-slot storage pilot
- tuple multi-payload compat: hidden payload boxes only

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
- `agg_local scalarization` now has a dedicated docs/facts entry phase
- the next follow-on after this docs cut is the actual `agg_local scalarization` layer work
