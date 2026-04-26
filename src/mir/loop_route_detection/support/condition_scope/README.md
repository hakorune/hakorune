# Route Detector Condition-Scope Support

This directory physically owns condition-scope analysis support for JoinIR route
planning.

Stable caller path:

```text
loop_route_detection::support::condition_scope
```

Rules:

- Keep variable extraction as a private helper of this family.
- Do not expose legacy `loop_condition_scope` or `condition_var_analyzer`
  names as caller paths.
- Keep route selection owned by `LoopFeatures -> classify() -> LoopRouteKind`.
