# Hako Alloc Segment Map Local-Free Reuse Ledger Bridge SSOT

Status: accepted
Decision: accepted
Date: 2026-05-18

## Purpose

MIMAP-192A proves the next scalar/model bridge after the segment-map local-free
reuse bridge closeout.

The route proves:

```text
segment-map released-span row
  -> modeled local-free reuse owner
  -> modeled local-free reuse ledger owner
```

It reuses the existing MIMAP-130A local-free reuse ledger owner. The row may
record a successful segment-map-derived reuse report as a deterministic live
reuse ledger row keyed by `(segment_id, page_id, reused_block_id)`. It does not
execute real segment allocation/free, real segment-map mutation, raw pointer
residence, real free-list mutation, or arena backing.

## Owners

Segment-map bridge owners:

```text
lang/src/hako_alloc/memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako
lang/src/hako_alloc/memory/segment_allocation_modeled_released_span_ledger_box.hako
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_box.hako
```

Reuse ledger owner:

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_box.hako
```

The row may:

- consume successful segment-map-derived local-free reuse reports;
- record one live reuse ledger row;
- prove duplicate, missing/upstream, and unsupported execution cases in
  scalar/model space;
- read the deterministic modeled reuse token and reused block id.

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
reuse=1,0,4,70007002
first=1,0,0,-1,70007004,70007002,70,7,4,5,6,1,1
duplicate=0,4,0
missing=0,1
unsupported=0,5
reads=70007004,4
counts=4,1,3,1,1,1,1,1
inactive=0,0,0,0,0,0,0,0,0,0,0
```

## Validation

MIMAP-192A is a daily L2 row:

```text
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_bridge_guard.sh
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
MIMAP-193A post-segment-map-local-free-reuse-ledger-bridge row selection
```
