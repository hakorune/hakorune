# 296x-1545: BOX-COMPILATION-CONTEXT-TYPED-OPERATION-IR-001

Status: landed

## Goal

Compile the bounded BoxCompilationContext easy-tier pilot into typed
operation IR and wire the generated artifact into the MirBuilder family route
lane.

## Slice

- Source: `crates/hakorune_mir_builder/src/context.rs`
- Family: `BoxCompilationContext`
- Bounded methods:
  - `new`
  - `is_empty`
- Excluded from the pilot:
  - `size_info`

## Notes

- The constructor is represented by multi-field box birth initialization.
- `is_empty` is compiled as a three-field conjunction over the internal
  ordered maps.
- The generated artifact is selected on the `derived_hako` route.
- `size_info` remains excluded from the bounded pilot.

## Acceptance

- `BoxCompilationContext::new` facts are represented as typed birth
  initialization for the three ordered-map fields
- `BoxCompilationContext::is_empty` facts are represented as a typed
  three-field conjunction
- The derived Hako artifact is generated and checked in
- The route selection SSOT is updated for the generated artifact
- MIR/EXE acceptance is green for the generated artifact
- `BoxCompilationContext::size_info` remains excluded from the pilot
