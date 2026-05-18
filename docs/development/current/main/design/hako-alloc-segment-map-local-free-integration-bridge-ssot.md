# Hako Alloc Segment Map Local-Free Integration Bridge SSOT

Status: accepted
Decision: accepted
Date: 2026-05-18

## Purpose

MIMAP-184A proves the next scalar/model bridge after the segment-map
local-free page-apply bridge closeout.

The route proves:

```text
segment-map released-span row
  -> modeled local-free integration owner
     -> local-free candidate ledger
     -> modeled local-free apply-plan ledger
     -> modeled local-free page-apply row
     -> explicit HakoAllocPageModel.releaseLocal(block_id)
```

It reuses the existing MIMAP-119A integration owner. MIMAP-184A may integrate
segment-map-derived released-span rows into the modeled local-free integration
report. It does not execute real segment free, real segment-map mutation, raw
pointer residence, real free-list mutation, or arena backing.

## Owners

Segment-map bridge owners:

```text
lang/src/hako_alloc/memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako
lang/src/hako_alloc/memory/segment_allocation_modeled_released_span_ledger_box.hako
```

Integration owner:

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_integration_box.hako
```

The row may:

- consume successful segment-map accepted-readiness rows;
- release modeled consume tokens into released-span rows;
- pass those released-span rows to `integrateLocalFree`;
- prove missing, duplicate, wrong-page, unsupported, and recycled integration
  cases in scalar/model space;
- mutate only the explicit `HakoAllocPageModel` through the existing
  page-apply owner.

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
first=1,0,0,0,0,70007002,70,7,2,5,3,3,6,3,0,3
missing=0,1,2
duplicate=0,1,3
wrong_page=0,3,4
unsupported=0,1,4
recycled=1,0,2,2,1,3,3
counts=6,2,4,3,0,1,3,3,2
page=3,2,3,5
inactive=0,0,0,0,0,0,0,0,0,0
```

## Validation

MIMAP-184A is a daily L2 row:

```text
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_integration_bridge_guard.sh
```

Representative L3 EXE evidence is deferred to a future closeout pack unless
this row introduces a new backend route shape.

## Stop Lines

- No real segment allocation/free execution.
- No raw pointer residence or pointer-derived lookup.
- No real segment-map mutation.
- No real free-list mutation.
- No direct page-array mutation outside explicit modeled page owners.
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
MIMAP-185A post-segment-map-local-free-integration-bridge row selection
```
