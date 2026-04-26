# Route Detector Local Support

This directory physically owns local-variable support helpers for JoinIR route
planning.

Stable caller paths:

```text
loop_route_detection::support::locals::pinned
loop_route_detection::support::locals::mutable_accumulator
```

Rules:

- Do not add route-selection policy here.
- Do not reintroduce legacy analyzer names as caller paths.
- Keep route selection owned by `LoopFeatures -> classify() -> LoopRouteKind`.
