# Hako Alloc Segment Map Released-Span Local-Free Candidate Bridge SSOT

Status: accepted
Decision: accepted
Date: 2026-05-18

## Purpose

MIMAP-172A proves the next scalar/model bridge after the segment-map
released-span observation closeout.

The route proves:

```text
segment-map modeled release report
  -> released-span ledger row
  -> modeled local-free candidate ledger row
```

It reuses the existing MIMAP-109A local-free candidate ledger. MIMAP-172A does
not mutate a free-list; it only proves that released-span facts produced by the
segment-map owner boundary can be consumed by the already-landed candidate
ledger.

## Owners

Source release/span owners:

```text
lang/src/hako_alloc/memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako
lang/src/hako_alloc/memory/segment_allocation_modeled_released_span_ledger_box.hako
```

Local-free candidate owner:

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_candidate_ledger_box.hako
```

The row may:

- record a successful segment-map released-span report into the existing
  `HakoAllocSegmentAllocationModeledLocalFreeCandidateLedger`;
- prove missing, duplicate, unsupported, and recycled local-free candidate
  cases in scalar/model space.

The row must not:

- execute real segment allocation/free;
- mutate a free-list;
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
span_first=1,0,0,-1,70007002,70,7,2,5,3
candidate_first=1,0,0,-1,70007002,70,7,2,5,3,1,1
candidate_missing=0,2,-1,1
candidate_duplicate=0,3,0,1
candidate_recycled=1,0,1,-1,70007002,70,7,2,5,3,2,2
candidate_unsupported=0,4,1
candidate_counts=5,2,2,3,0,1,1,1
inactive=0,0,0,0,0,0,0,0,0,0
```

## Validation

MIMAP-172A is a daily L2 row:

```text
bash tools/checks/k2_wide_hako_alloc_segment_map_released_span_local_free_candidate_bridge_guard.sh
```

Representative L3 EXE evidence is deferred to a future closeout pack unless
this row introduces a new backend route shape.

## Stop Lines

- No real segment allocation/free execution.
- No raw pointer residence or pointer-derived lookup.
- No real segment-map mutation.
- No free-list mutation.
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
MIMAP-173A post-segment-map-released-span-local-free-candidate-bridge row selection
```
