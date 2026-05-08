# User-Box Method Route Plan

This directory keeps MIR-owned user-box method route planning split by
responsibility.

- `origin_inference.rs`: infers param and field box origins from MIR metadata,
  value origins, typed object plans, and already-published route facts.
- `return_shape.rs`: infers same-module user-box method return shape.
- `value_type_publish.rs`: publishes route-derived value types back into MIR
  metadata for later backend consumers.
- `tests.rs`: unit tests for the facade in `../user_box_method_route_plan.rs`.

The C shim must consume the resulting metadata. It must not rediscover these
origins or infer user-box method legality from `.hako` source shape.
