# nyash_mir_defs

Shared MIR call-shape substrate extracted during the crate split preparation
lane.

## Scope

- `call_unified.rs`

## Boundaries

- This crate only holds unified call definitions and related call-shape helpers.
- It depends on `nyash_mir_core` for the pure substrate types.
- It does not own MIR lowering, builder policy, or bridge routing.

