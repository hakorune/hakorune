# Route Detector Function-Scope Support

This directory provides function-scope capture types for JoinIR route support.

Stable caller path:

```text
loop_route_detection::support::function_scope
```

Rules:

- Keep route selection policy outside this directory.
- Do not expose legacy `function_scope_capture` names as caller paths.
- Keep route selection owned by `LoopFeatures -> classify() -> LoopRouteKind`.
