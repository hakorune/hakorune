# Hako Alloc Segment Map Local-Free Page-Apply Bridge SSOT

Status: accepted
Decision: accepted
Date: 2026-05-18

## Purpose

MIMAP-180A proves the next scalar/model bridge after the segment-map
local-free apply-plan bridge closeout.

The route proves:

```text
segment-map released-span row
  -> local-free candidate ledger row
  -> modeled local-free apply-plan ledger row
  -> modeled local-free page-apply row
```

It reuses the existing MIMAP-115A page-apply owner. MIMAP-180A may apply the
modeled local-free span to an explicit `HakoAllocPageModel` through
`releaseLocal`; it does not mutate a real allocator free-list, raw pointer
residence, or real segment-map state.

## Owners

Bridge owners:

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_released_span_ledger_box.hako
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_candidate_ledger_box.hako
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_apply_plan_box.hako
```

Page-apply owner:

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_page_apply_box.hako
```

The row may:

- consume a successful segment-map-derived local-free apply-plan report;
- call `HakoAllocPageModel.releaseLocal(block_id)` on the explicit modeled
  page;
- prove missing, duplicate, wrong-page, unsupported, and recycled page-apply
  cases in scalar/model space.

The row must not:

- execute real segment allocation/free;
- mutate real allocator free-lists;
- mutate page arrays directly;
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
plan_first=1,0,0,-1,0,70007002,70,7,2,5,3
page_first=1,0,0,-1,0,70007002,70,7,2,5,3,3,6,3,0,3
page_missing=0,2,-1,1
page_duplicate=0,3,0,1
page_wrong=0,4,1
page_unsupported=0,6,1
page_recycled=1,0,1,-1,1,70007002,70,7,2,5,3,3,6,3,0,3
page_counts=6,2,2,4,0,1,1,1,0,1
page_state=3,2,3,5
inactive=0,0,0,0,0,0,0,0,0,0
```

## Validation

MIMAP-180A is a daily L2 row:

```text
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_page_apply_bridge_guard.sh
```

Representative L3 EXE evidence is deferred to a future closeout pack unless
this row introduces a new backend route shape.

## Stop Lines

- No real segment allocation/free execution.
- No raw pointer residence or pointer-derived lookup.
- No real segment-map mutation.
- No real free-list mutation.
- No direct page-array mutation outside `HakoAllocPageModel.releaseLocal`.
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
MIMAP-181A post-segment-map-local-free-page-apply-bridge row selection
```
