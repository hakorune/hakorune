# Typed Object Storage Inference

This directory keeps TypedObjectPlan storage inference split by responsibility.

- `../storage_inference.rs`: facade and fixed-point loops that infer field,
  parameter, and collection element storage.
- `value_analysis.rs`: recursive value storage and box-origin analysis used by
  the fixed-point loops.
- `tests.rs`: storage inference unit tests.

TypedObjectPlan remains the MIR-owned layout truth for backends. C shims should
consume the emitted plan and must not rediscover field storage from source
shape.
