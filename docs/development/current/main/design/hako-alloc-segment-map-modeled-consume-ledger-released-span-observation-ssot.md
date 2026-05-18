# Hako Alloc Segment Map Modeled Consume Ledger Released-Span Observation SSOT

Status: accepted
Decision: accepted
Date: 2026-05-18

## Purpose

MIMAP-168A proves released-span observation at the segment-map modeled
consume-ledger owner boundary.

The route proves:

```text
accepted explicit-ID readiness
  -> segment-map modeled consume-ledger token
  -> segment-map modeled release report
  -> released-span ledger records the scalar span facts
```

It reuses the existing MIMAP-107A released-span ledger. MIMAP-168A only adds
the missing scalar `modeled_block_end` field to the segment-map release report
so the already-landed released-span ledger can validate the span shape.

## Owners

Source release report:

```text
lang/src/hako_alloc/memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako
```

Released-span ledger:

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_released_span_ledger_box.hako
```

The row may:

- expose `modeled_block_end` on
  `HakoAllocSegmentMapModeledConsumeLedgerReleaseReport`;
- record a successful segment-map release report into the existing
  `HakoAllocSegmentAllocationModeledReleasedSpanLedger`;
- prove missing, duplicate, unsupported, and recycled released-span observation
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
release_first=1,0,0,70007002,70,7,2,5,3,1
first=1,0,0,-1,70007002,70,7,2,5,3,1,1
missing=0,2,-1,1
duplicate=0,3,0,1
recycled=1,0,1,-1,70007002,70,7,2,5,3,2,2
unsupported=0,4,1
counts=5,2,2,3,0,1,1,1
inactive=0,0,0,0,0,0,0,0,0,0
```

## Validation

MIMAP-168A is a daily L2 row:

```text
bash tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_released_span_observation_guard.sh
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
MIMAP-169A post-segment-map-modeled-consume-ledger-released-span-observation row selection
```
