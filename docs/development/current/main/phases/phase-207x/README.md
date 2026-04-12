# Phase 207x: generic placement / effect docs-facts phase

Status: Active

Purpose
- inventory the current pilot scaffolds under the generic placement / effect layer
- fix the first docs/facts boundary before any code widening

Scope
- align current pointers with the roadmap SSOT
- inventory the current pilot surfaces: string corridor candidates, sum placement chains, and thin-entry selection
- define the first owner boundary for the generic placement / effect layer

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
- the next follow-on after this docs cut is `agg_local scalarization`
