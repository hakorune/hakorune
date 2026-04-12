# 221x-90 Generic Placement/Effect First MIR-Side Transform Cut SSOT

Status: Landed

Goal
- capture the first MIR-side generic placement/effect implementation slice after the route-window helper polish

Why this cut
- `phase-219x` proved the route-window read on `placement_effect_routes`
- `phase-220x` kept the route-window branch thin without changing behavior
- the next useful step was the first MIR-side transform cut that consumed the shared inventory

Scope
- kept the first MIR-side generic transform slice narrow
- preserved the family-specific fallback paths while the new slice proved out
- avoided growing another family-specific route walker

Non-goals
- no new route kind
- no backend-wide widening
- no memory-effect layer work
- no simplification-bundle work

Exit
- the next implementation phase can be started without ambiguity about owner or scope

Landing
- MIR refresh now materializes `metadata.string_kernel_plans` as the first first-class generic placement/effect transform slice
- the JSON emitter reads the precomputed metadata only
- family-specific fallbacks remain as compatibility paths
