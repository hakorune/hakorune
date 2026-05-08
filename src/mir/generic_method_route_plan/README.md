# Generic Method Route Plan

This directory keeps MIR-owned generic method route planning split by
responsibility.

- `../generic_method_route_plan.rs`: facade and route matcher orchestration.
- `origin_inference.rs`: typed-object field handle and collection element
  origin inference.
- `flow_origin.rs`: collection/string value-flow origin observation.
- `write_routes.rs`: mutating generic method route matchers (`push`, `set`,
  `delete`).
- `mir_json_routes.rs`: MIR JSON-specific generic `get` route shapes.
- `map_set_scalar_proof.rs`: scalar map get/store proof.
- `model.rs`: route data model consumed by MIR JSON and backend emitters.

Backends consume these route facts. They must not rediscover collection,
string, or typed-object flow legality from source names or `.hako` shape.
