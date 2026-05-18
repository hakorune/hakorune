# Hako Alloc Segment Map Released-Span Local-Free Candidate Bridge Closeout SSOT

Status: accepted
Decision: accepted
Date: 2026-05-18

## Purpose

Close out the segment-map released-span local-free candidate bridge pack opened
by MIMAP-172A.

The closed pack is:

```text
MIMAP-172A segment-map released-span local-free candidate bridge
```

The pack freezes the bridge from segment-map released-span rows into the
existing MIMAP-109A local-free candidate ledger before opening real free-list
mutation, raw pointer residence, arena backing, real segment-map execution, or
atomic bitmap behavior.

## Validation Pack

Pack id:

```text
segment-map-local-free-candidate-bridge
```

Daily validation remains L2:

```text
bash tools/checks/run_proof_app.sh --closeout-pack segment-map-local-free-candidate-bridge --level L2
```

MIMAP-174A owns the representative L3 EXE evidence:

```text
bash tools/checks/k2_wide_hako_alloc_segment_map_released_span_local_free_candidate_bridge_closeout_guard.sh
```

The L3 guard must build the EXE from the exact MIR artifact after route
preflight and verify released-span production, local-free candidate recording,
missing report rejection, duplicate rejection, unsupported execution rejection,
recycled-token candidate recording, and inactive substrate flags.

## Stop Lines

MIMAP-174A must not add:

- real segment allocation/free execution;
- real free-list mutation;
- raw pointer residence or pointer-derived lookup;
- real segment-map mutation;
- arena backing allocation;
- atomic bitmap execution;
- OSVM/page-source execution;
- TLS, worker-local, worker scheduling, or source-level concurrency;
- provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`;
- cross-function `Result` direct ABI or runtime sum materialization;
- backend helper/app/owner name matchers.

## Next Row

After closeout, the selected row is:

```text
MIMAP-175A post-segment-map-released-span-local-free-candidate-bridge-closeout row selection
```
