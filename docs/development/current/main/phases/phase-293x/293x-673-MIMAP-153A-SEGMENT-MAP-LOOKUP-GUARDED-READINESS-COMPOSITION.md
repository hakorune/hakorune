# 293x-673 MIMAP-153A Segment Map Lookup Guarded Readiness Composition

Status: selected current
Date: 2026-05-18

## Decision

Add a proof-only composition owner that gates segment/page membership and
allocation readiness behind the explicit-ID segment-map scalar lookup from
MIMAP-151A.

## Owner

```text
lang/src/hako_alloc/memory/
apps/hako-alloc-segment-map-lookup-guarded-readiness-composition-proof/
```

## Scope

- Compose:
  - `HakoAllocSegmentMapScalarLookupBoundaryInventory`,
  - `HakoAllocSegmentPageMembershipScalar`,
  - `HakoAllocSegmentAllocationReadinessScalar`.
- Prove one accepted lookup -> membership -> readiness path.
- Prove stable reject paths for lookup reject, membership reject, readiness
  reject, and raw-pointer lookup request.
- Keep the route proof-only and executable in VM and pure-first EXE.

## Stop Lines

- No raw pointer residence or pointer-derived lookup.
- No real segment-map execution.
- No real segment allocation/free.
- No arena backing allocation.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No worker scheduling, provider activation, host allocator replacement, hooks,
  or `#[global_allocator]`.
- No backend `.inc` matcher by app or owner name.

## Required Evidence

```text
bash tools/checks/run_proof_app.sh --only MIMAP-153A
bash tools/checks/k2_wide_hako_alloc_segment_map_lookup_guarded_readiness_composition_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
