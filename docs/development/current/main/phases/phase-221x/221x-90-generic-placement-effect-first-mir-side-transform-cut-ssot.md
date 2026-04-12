# 221x-90 Generic Placement/Effect First MIR-Side Transform Cut SSOT

Status: Planned

Goal
- capture the next MIR-side generic placement/effect implementation slice after the route-window helper polish

Why this cut
- `phase-219x` proved the route-window read on `placement_effect_routes`
- `phase-220x` kept the route-window branch thin without changing behavior
- the next useful step is the first MIR-side transform cut that consumes the shared inventory

Scope
- keep the first MIR-side generic transform slice narrow
- preserve the family-specific fallback paths while the new slice proves out
- avoid growing another family-specific route walker

Non-goals
- no new route kind
- no backend-wide widening
- no memory-effect layer work
- no simplification-bundle work

Exit
- the next implementation phase can be started without ambiguity about owner or scope
