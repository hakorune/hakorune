# 221x-90 Generic Placement/Effect First MIR-Side Transform Cut SSOT

Status: Landed

Goal
- capture the first MIR-side generic placement/effect implementation slice after the route-window helper polish

Landing
- MIR refresh now materializes `metadata.string_kernel_plans`
- the JSON emitter reads the precomputed metadata only
- family-specific fallbacks remain as compatibility paths

Exit
- the next implementation phase can start without ambiguity about owner or scope
