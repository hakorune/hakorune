# Route Detector Function-Scope Support

This directory physically owns function-scope capture support for JoinIR route
planning.

Stable caller path:

```text
loop_route_detection::support::function_scope
```

Rules:

- Keep capture analysis here; do not move route selection policy into this
  directory.
- Do not expose legacy `function_scope_capture` names as caller paths.
- Keep route selection owned by `LoopFeatures -> classify() -> LoopRouteKind`.
