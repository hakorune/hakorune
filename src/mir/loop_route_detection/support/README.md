# Route Detector Support

This directory owns stable caller paths for route detector support helpers.

`legacy/` is private implementation storage. New non-legacy callers must use
semantic support modules here.

Current stable owner paths:

```text
loop_route_detection::support::condition_scope
loop_route_detection::support::function_scope
loop_route_detection::support::trim
loop_route_detection::support::body_local::{carrier, condition}
loop_route_detection::support::break_condition
loop_route_detection::support::locals::{pinned, mutable_accumulator}
```

Physically owned here:

```text
loop_route_detection::support::break_condition
loop_route_detection::support::trim
loop_route_detection::support::function_scope
loop_route_detection::support::condition_scope
loop_route_detection::support::locals::{pinned, mutable_accumulator}
```

Do not add route-selection policy here. Current route selection remains:

```text
LoopFeatures -> classify() -> LoopRouteKind
```

Current implementation note: remaining support modules are still re-export
facades over private `legacy/` storage. Move files into this directory
family-by-family, after each family has a focused validation slice.

Do not expose `legacy` again to make a migration easier.
