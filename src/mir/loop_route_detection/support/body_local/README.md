# Route Detector Body-Local Support

This directory physically owns loop-body-local promotion support for JoinIR
route planning.

Stable caller paths:

```text
loop_route_detection::support::body_local::carrier
loop_route_detection::support::body_local::condition
```

Private helpers:

```text
digitpos
digitpos_detector
trim_detector
```

Rules:

- Keep detector helpers private to this family unless a stable support caller
  path is explicitly needed.
- Do not expose legacy promoter/detector names as caller paths.
- Keep route selection owned by `LoopFeatures -> classify() -> LoopRouteKind`.
