# Phase 221x: generic placement/effect first MIR-side transform cut

Status: Planned

Purpose
- start the first MIR-side generic transform cut after the route-window helper polish
- keep the next behavior slice narrow and rooted in the shared placement/effect inventory

Scope
- define the first MIR-side transform slice that consumes the shared placement/effect inventory
- keep family-specific fallbacks intact until the slice proves green
- keep the cut narrow and behavior-preserving

Non-goals
- no new route kind
- no broad widening of string / sum / user-box / array / map
- no DCE / simplification-bundle work
- no memory-effect / escape / closure work

Acceptance
- the next implementation slice is named and bounded
- the smoke/gate plan is explicit
- no behavior-changing code is required yet

Follow-on
- `semantic simplification bundle` remains the next major layer after the first MIR-side generic transform cut
