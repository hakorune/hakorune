# Hako Alloc Segment Map Modeled Consume Ledger Released-Token Recycle SSOT

Status: accepted
Decision: accepted
Date: 2026-05-18

## Purpose

MIMAP-164A proves released-token recycle at the segment-map modeled
consume-ledger owner boundary.

The route proves:

```text
accepted explicit-ID readiness
  -> modeled consume ledger live token
  -> modeled release through the same owner
  -> same scalar token may become live again as a new row
```

It reuses the existing `HakoAllocSegmentAllocationModeledLedger` released-token
recycle behavior from MIMAP-100A. MIMAP-164A does not add a second recycle
ledger and does not open real segment-map allocation.

## Owner

```text
lang/src/hako_alloc/memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako
```

The owner may:

- accept a modeled consume row from an accepted MIMAP-153A readiness report;
- reject a simultaneous live duplicate at the segment-map consume-ledger
  boundary;
- release the live token through `releaseConsumedToken`;
- accept the same scalar token again as a new live row after release;
- expose scalar counters and read-only report fields proving the recycle.

The owner must not:

- execute real segment allocation or free;
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
first=1,0,0,-1,70007002,1,1
duplicate_live=0,3,4,0,2
release_first=1,0,0,1,0,0
after_release=-1,0,-1
recycled=1,0,1,-1,70007002,2,1
after_recycle=1,0,1,70007002
duplicate_after_recycle=0,3,4,1
release_recycled=1,0,1,1,0,0
counts=4,2,2,2,2,2,0,2,2,0,70007002,0
inactive=0,0,0,0,0,0,0,0,0,0
```

## Validation

MIMAP-164A is a daily L2 row:

```text
bash tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_released_token_recycle_guard.sh
```

Representative L3 EXE evidence is deferred to a future closeout pack unless
this row introduces a new backend route shape.

## Stop Lines

- No real segment allocation/free execution.
- No raw pointer residence or pointer-derived lookup.
- No real segment-map mutation.
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
MIMAP-165A post-segment-map-modeled-consume-ledger-released-token-recycle row selection
```
