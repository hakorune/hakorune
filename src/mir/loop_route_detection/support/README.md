# Route Detector Support

This directory owns stable caller paths for route detector support helpers.

New non-legacy callers must use semantic support modules here.

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
loop_route_detection::support::body_local::{carrier, condition}
loop_route_detection::support::locals::{pinned, mutable_accumulator}
```

Do not add route-selection policy here. Current route selection remains:

```text
LoopFeatures -> classify() -> LoopRouteKind
```

All former route-detector support facades are now physically owned under this
directory. Do not expose legacy module names again to make future migration
work easier.
