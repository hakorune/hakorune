# Private Route Detector Storage

This directory is private implementation storage for route detector support
helpers.

Do not add new non-legacy caller paths here. Public callers should use:

```text
crate::mir::loop_route_detection::support::...
```

Current route selection remains owned by the classifier surface:

```text
LoopFeatures -> classify() -> LoopRouteKind
```

## Rules

- Keep `legacy` private in `src/mir/loop_route_detection/mod.rs`.
- Do not re-export legacy-named modules from the parent module.
- Internal helpers may use `super::...` owner-local paths.
- New route-selection policy belongs in `features` / `classify`, not here.
- Physical moves out of this directory must be family-sized and validated with
  focused `cargo check -q`.

## Migration Direction

The semantic owner paths are already available under `support/`.

Move low-risk families first:

```text
support::break_condition
support::locals::{pinned, mutable_accumulator}
```

Then move medium/high dependency families:

```text
support::trim
support::function_scope
support::condition_scope
support::body_local::{carrier, condition}
```
