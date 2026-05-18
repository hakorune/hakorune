# Hako Alloc Segment Map Local Free Reuse Ledger Bridge Closeout SSOT

Status: active
Date: 2026-05-18
Decision: accepted

## Purpose

Close the segment-map local-free reuse ledger bridge pack with representative
exact-MIR L3 EXE evidence.

MIMAP-192A proves the daily L2 behavior:

```text
segment-map local-free reuse report
  -> modeled local-free reuse ledger owner
```

MIMAP-194A keeps that behavior in the
`segment-map-local-free-reuse-ledger-bridge` pack and runs representative L3 EXE evidence for the pack.

## Validation

Daily validation remains L2:

```bash
bash tools/checks/run_proof_app.sh --closeout-pack segment-map-local-free-reuse-ledger-bridge --level L2 --dry-run
```

Closeout validation runs representative L3 EXE:

```bash
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_bridge_closeout_guard.sh
```

The closeout guard must:

- dry-run the closeout pack selection and include `MIMAP-192A`;
- run the MIMAP-192A L2 guard;
- emit MIR once, build EXE from the exact MIR artifact, and run the EXE;
- assert VM and EXE output parity for the ledger bridge proof app;
- keep `.inc` matcher growth at zero.

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

```text
MIMAP-195A post-segment-map-local-free-reuse-ledger-bridge-closeout row selection
```
