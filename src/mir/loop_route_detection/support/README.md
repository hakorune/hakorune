# Route Detector Support Facades

This directory owns stable caller paths for route detector support helpers.

`legacy/` is private implementation storage. New non-legacy callers should use
semantic support modules instead of `loop_route_detection::{legacy module name}`.

Current stable owner paths:

```text
loop_route_detection::support::condition_scope
loop_route_detection::support::function_scope
loop_route_detection::support::trim
loop_route_detection::support::body_local::{carrier, condition}
loop_route_detection::support::break_condition
loop_route_detection::support::locals::{pinned, mutable_accumulator}
```

Do not add route-selection policy here. Current route selection remains:

```text
LoopFeatures -> classify() -> LoopRouteKind
```

Removal condition: once callers are migrated and files are physically moved out
of `legacy/`, these facades can become real owner modules instead of re-export
facades.
