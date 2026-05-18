# Hako Alloc Segment Map Local-Free Page-Apply Bridge Closeout SSOT

Status: accepted
Decision: accepted
Date: 2026-05-18

## Purpose

MIMAP-182A closes the `segment-map-local-free-page-apply-bridge` validation
pack opened by MIMAP-180A.

The closeout proves that the already-landed bridge remains stable across:

```text
segment-map released-span row
  -> local-free candidate ledger row
  -> modeled local-free apply-plan ledger row
  -> modeled local-free page-apply row
```

It adds representative L3 EXE evidence for the pack without opening real
allocator free-list mutation, raw pointer residence, real segment-map
execution, arena backing, or atomic bitmap behavior.

## Validation

Daily L2 selection remains:

```text
bash tools/checks/run_proof_app.sh --closeout-pack segment-map-local-free-page-apply-bridge --level L2
```

MIMAP-182A owns the representative L3 EXE evidence:

```text
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_page_apply_bridge_closeout_guard.sh
```

The closeout guard must:

- select the `segment-map-local-free-page-apply-bridge` pack through the
  manifest runner;
- run the MIMAP-180A proof app on VM;
- emit the exact MIR artifact used for EXE build;
- run the resulting EXE and assert the same visible proof output;
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
MIMAP-183A post-segment-map-local-free-page-apply-bridge-closeout row selection
```
