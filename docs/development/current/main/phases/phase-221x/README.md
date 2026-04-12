# Phase 221x: generic placement/effect first MIR-side transform cut

Status: Landed

Purpose
- land the first MIR-side generic transform cut after the route-window helper polish
- keep the next behavior slice narrow and rooted in the shared placement/effect inventory

Scope
- defined and landed the first MIR-side transform slice that consumes the shared placement/effect inventory
- kept family-specific fallbacks intact while the slice proved green
- kept the cut narrow and behavior-preserving

Non-goals
- no new route kind
- no broad widening of string / sum / user-box / array / map
- no DCE / simplification-bundle work
- no memory-effect / escape / closure work

Acceptance
- the first implementation slice is named and bounded
- the smoke/gate plan is explicit
- no behavior-changing code was required

Landing
- the MIR-side refresh now materializes the first generic placement/effect `StringKernelPlan` inventory
- the JSON emitter serializes that inventory instead of deriving it on the fly
- family-specific fallbacks remain in place as thin compatibility paths

Follow-on
- `semantic simplification bundle` remains the next major layer after the first MIR-side generic transform cut
