# 293x-686 MIMAP-164A Segment Map Modeled Consume Ledger Released Token Recycle Route

Status: landed
Date: 2026-05-18

## Decision

Add a segment-map owner-boundary proof that a released modeled consume-ledger
token can be recorded again as a new live row.

## Context

MIMAP-161A proved release through the segment-map modeled consume-ledger owner.
MIMAP-164A keeps the same owner and proves the next model-only step:

```text
accepted readiness
  -> consume-ledger token live
  -> release token
  -> same scalar token accepted again as a new live row
```

This mirrors the lower-level MIMAP-100A released-token recycle contract without
opening real segment-map allocation/free execution.

## Scope

- Add the proof app
  `apps/hako-alloc-segment-map-modeled-consume-ledger-released-token-recycle-proof/`.
- Add the design SSOT
  `docs/development/current/main/design/hako-alloc-segment-map-modeled-consume-ledger-released-token-recycle-ssot.md`.
- Add the L2 guard
  `tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_released_token_recycle_guard.sh`.
- Register the proof app in `tools/checks/proof_apps.toml`.

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

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_released_token_recycle_guard.sh
bash tools/checks/run_proof_app.sh --only MIMAP-164A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIMAP-165A post-segment-map-modeled-consume-ledger-released-token-recycle row selection
```
