# Hako Alloc Segment Map Local-Free Reuse Bridge Closeout SSOT

Status: accepted
Decision: accepted
Date: 2026-05-18

## Purpose

MIMAP-190A closes the `segment-map-local-free-reuse-bridge` validation pack
opened by MIMAP-188A.

The closeout proves that the already-landed bridge remains stable across:

```text
segment-map released-span row
  -> modeled local-free integration owner
  -> modeled local-free reuse owner
     -> HakoAllocPageModel.acquire(size)
```

It adds representative L3 EXE evidence for the pack without opening real
allocator free-list mutation, raw pointer residence, real segment-map
execution, arena backing, or atomic bitmap behavior.

The closeout also keeps the reuse owner report construction local to the owner
instead of routing the large report payload through a wide helper argument
list. This is a source-structure cleanup for exact-MIR EXE parity, not a new
allocator behavior.

## Validation

Daily L2 selection remains:

```text
bash tools/checks/run_proof_app.sh --closeout-pack segment-map-local-free-reuse-bridge --level L2
```

MIMAP-190A owns the representative L3 EXE evidence:

```text
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_bridge_closeout_guard.sh
```

The closeout guard must:

- select the `segment-map-local-free-reuse-bridge` pack through the manifest
  runner;
- run the MIMAP-188A proof app on VM;
- emit the exact MIR artifact used for EXE build;
- run the resulting EXE and assert the same visible proof output;
- keep `HakoAllocSegmentAllocationModeledLocalFreeReuse` report construction
  behind a local `finishReport` helper instead of a wide report-argument
  helper;
- reject provider/hook/global allocator activation and `.inc` app/name
  matchers.

## Stop Lines

- No real segment allocation/free execution.
- No real free-list mutation.
- No direct page-array mutation outside explicit modeled page owners.
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
MIMAP-191A post-segment-map-local-free-reuse-bridge-closeout row selection
```
