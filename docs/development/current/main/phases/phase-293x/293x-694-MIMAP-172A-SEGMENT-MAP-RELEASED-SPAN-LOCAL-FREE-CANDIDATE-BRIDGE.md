# 293x-694 MIMAP-172A Segment Map Released Span Local Free Candidate Bridge

Status: landed
Date: 2026-05-18

## Decision

Add a scalar/model bridge from segment-map released-span observation into the
existing local-free candidate ledger.

## Context

MIMAP-170A closed released-span observation at the segment-map modeled
consume-ledger boundary. MIMAP-172A keeps the chain in scalar/model space and
proves the next existing owner can consume those released-span facts:

```text
segment-map release report
  -> released-span ledger
  -> local-free candidate ledger
```

This prepares modeled free-list planning without mutating a real free-list.

## Scope

- Add proof app:
  `apps/hako-alloc-segment-map-released-span-local-free-candidate-bridge-proof/`.
- Add SSOT:
  `docs/development/current/main/design/hako-alloc-segment-map-released-span-local-free-candidate-bridge-ssot.md`.
- Add L2 guard:
  `tools/checks/k2_wide_hako_alloc_segment_map_released_span_local_free_candidate_bridge_guard.sh`.

## Stop Lines

- No real segment allocation/free execution.
- No free-list mutation.
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
bash tools/checks/k2_wide_hako_alloc_segment_map_released_span_local_free_candidate_bridge_guard.sh
bash tools/checks/run_proof_app.sh --only MIMAP-172A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIMAP-173A post-segment-map-released-span-local-free-candidate-bridge row selection
```
