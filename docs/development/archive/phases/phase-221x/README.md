# Phase 221x: generic placement/effect first MIR-side transform cut

Status: Landed

Purpose
- land the first MIR-side generic transform cut after the route-window helper polish

Landing
- MIR refresh now materializes `metadata.string_kernel_plans`
- the JSON emitter serializes that inventory instead of deriving it on the fly
- family-specific fallbacks remain as compatibility paths

Follow-on
- `semantic simplification bundle`
