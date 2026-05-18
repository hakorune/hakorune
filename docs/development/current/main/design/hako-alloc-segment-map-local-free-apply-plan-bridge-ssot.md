# Hako Alloc Segment Map Local-Free Apply-Plan Bridge SSOT

Status: accepted
Decision: accepted
Date: 2026-05-18

## Purpose

MIMAP-176A proves the next scalar/model bridge after the segment-map
local-free candidate bridge closeout.

The route proves:

```text
segment-map released-span row
  -> local-free candidate ledger row
  -> modeled local-free apply-plan ledger row
```

It reuses the existing MIMAP-111A local-free apply-plan ledger. MIMAP-176A
does not mutate a free-list or page state; it only proves that segment-map
derived candidate facts can be consumed by the already-landed apply-plan
ledger.

## Owners

Source bridge owners:

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_released_span_ledger_box.hako
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_candidate_ledger_box.hako
```

Apply-plan owner:

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_apply_plan_box.hako
```

The row may:

- record a successful segment-map-derived local-free candidate report into the
  existing `HakoAllocSegmentAllocationModeledLocalFreeApplyPlan`;
- prove missing, duplicate, unsupported, and recycled apply-plan cases in
  scalar/model space.

The row must not:

- execute real segment allocation/free;
- mutate a free-list;
- mutate page state;
- allocate arena backing;
- use raw pointer residence;
- perform real segment-map lookup or mutation;
- claim or unclaim an atomic bitmap;
- call page-source / OSVM seams;
- schedule or spawn workers;
- activate providers, hooks, host allocator replacement, or
  `#[global_allocator]`;
- add backend `.inc` app/name matchers.

## Acceptance Shape

The proof app must expose at least:

```text
candidate_first=1,0,0,-1,70007002,70,7,2,5,3
plan_first=1,0,0,-1,0,70007002,70,7,2,5,3,1,1,1
plan_missing=0,2,-1,1
plan_duplicate=0,3,0,1
plan_recycled=1,0,1,-1,1,70007002,70,7,2,5,3,1,2,2
plan_unsupported=0,5,1
plan_counts=5,2,2,3,0,1,1,0,1
inactive=0,0,0,0,0,0,0,0,0,0,0
```

## Validation

MIMAP-176A is a daily L2 row:

```text
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_apply_plan_bridge_guard.sh
```

Representative L3 EXE evidence is deferred to a future closeout pack unless
this row introduces a new backend route shape.

## Stop Lines

- No real segment allocation/free execution.
- No raw pointer residence or pointer-derived lookup.
- No real segment-map mutation.
- No free-list mutation.
- No page-state mutation.
- No arena backing allocation.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app or owner name.

## Next

The next selected row is:

```text
MIMAP-177A post-segment-map-local-free-apply-plan-bridge row selection
```
