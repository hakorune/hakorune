# Phase 218x: shared placement effect route reader seam

Status: Landed

Purpose
- keep `generic placement / effect` moving without growing more family-specific route walkers
- extract one shared folded-route reader seam for current C shim consumers

Scope
- reuse one common `placement_effect_routes` reader/matcher in the C shim layer
- switch the current sum local seed metadata helpers to that shared reader
- keep legacy `thin_entry_selections` / `sum_placement_*` fallbacks intact

Non-goals
- no new placement/effect semantic widening
- no string folded-route migration
- no fallback removal
- no MIR-side transform in this cut

Acceptance
- C shim folded-route matching is shared instead of reimplemented per family
- sum boundary helper behavior stays green on the current pure-first owner lane
- user-box boundary helper behavior stays green on the same shared reader seam

Follow-on
- the next `generic placement / effect` cut should either fold another remaining boundary/backend consumer into the shared seam or start the first MIR-side generic transform cut
