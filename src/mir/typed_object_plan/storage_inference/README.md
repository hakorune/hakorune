# Typed Object Storage Inference

This directory keeps TypedObjectPlan storage inference split by responsibility.

- `../storage_inference.rs`: sole allocation orchestration and compatibility
  inference; source-issued canonical membership is excluded from field inference.
- `canonical_layout.rs`: declaration-only canonical layouts. Every definition
  reserves a checked ID position, even when unavailable. Compatibility IDs follow
  that prefix; repeated refresh validates canonical allocations without replacing
  them. The canonical definition owns its layout; metadata is a one-way legacy
  projection, checked for drift, reserved-ID intrusion and duplicates before refresh.
- `value_analysis.rs`: recursive value storage and box-origin analysis used by
  the fixed-point loops.
- `tests.rs`: storage inference unit tests.

Refresh returns errors through compiler/Raw postprocess/backend boundaries.
Preparation is whole-module: no layout or projection is installed on failure.
The initial canonical field subset is exact numeric; weak/unresolved fields and
unsupported declaration structure stay explicitly unavailable, never MIR-inferred.
This does not activate Birth, Home release or construction cleanup.

TypedObjectPlan remains MIR-owned physical layout data for backends. C shims should
consume the emitted plan and must not rediscover field storage from source
shape.
