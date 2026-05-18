# Hako Alloc Segment Map Local-Free Apply-Plan Bridge Closeout SSOT

Status: accepted
Decision: accepted
Date: 2026-05-18

## Purpose

MIMAP-178A closes the `segment-map-local-free-apply-plan-bridge` validation
pack opened by MIMAP-176A.

The closeout proves that the already-landed bridge remains stable across:

```text
segment-map released-span row
  -> local-free candidate ledger row
  -> modeled local-free apply-plan ledger row
```

It adds representative L3 EXE evidence for the pack without opening real
free-list mutation, page-state mutation, raw pointer residence, or real
segment-map execution.

## Validation

Daily L2 selection remains:

```text
bash tools/checks/run_proof_app.sh --closeout-pack segment-map-local-free-apply-plan-bridge --level L2
```

MIMAP-178A owns the representative L3 EXE evidence:

```text
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_apply_plan_bridge_closeout_guard.sh
```

The closeout guard must:

- select the `segment-map-local-free-apply-plan-bridge` pack through the
  manifest runner;
- run the MIMAP-176A proof app on VM;
- emit the exact MIR artifact used for EXE build;
- run the resulting EXE and assert the same visible proof output;
- reject provider/hook/global allocator activation and `.inc` app/name
  matchers.

## Stop Lines

- No real segment allocation/free execution.
- No real free-list mutation.
- No page-state mutation.
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
MIMAP-179A post-segment-map-local-free-apply-plan-bridge-closeout row selection
```
