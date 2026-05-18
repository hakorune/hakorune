# 293x-673 MIMAP-153A Segment Map Lookup Guarded Readiness Composition

Status: landed
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

## Implementation

- Added `HakoAllocSegmentMapLookupGuardedReadinessComposition` as the
  proof-only composition owner.
- Added a proof app that accepts the lookup -> membership -> readiness path and
  rejects lookup, membership, readiness, and raw-pointer request paths with
  stable subreasons.
- Registered the owner in the `hako_alloc` module, proof manifest, check
  script index, allocator docs, and current task docs.

## Closeout

MIMAP-153A landed with this proof surface:

```text
composition=1,0,0,0,0,70,7,3,16,1,2,8,3,6
reject_reasons=1,2,3,1
subreasons=3,8,2,2
inactive=0,0,0,0,0,0,0,0,0
counts=5,1,4,2,1,1,1,1
```

The next selected row is `MIMAP-154A`, a post-lookup-guarded-readiness row
selection. Raw pointer residence, real segment-map execution, arena backing,
atomic bitmap execution, OSVM execution, thread scheduling, provider
activation, and backend matchers remain closed.
