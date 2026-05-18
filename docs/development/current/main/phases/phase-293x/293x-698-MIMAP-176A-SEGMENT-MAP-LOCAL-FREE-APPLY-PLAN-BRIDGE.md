# 293x-698 MIMAP-176A Segment Map Local Free Apply Plan Bridge

Status: landed
Date: 2026-05-18

## Decision

Add a scalar/model bridge from segment-map local-free candidate rows into the
existing local-free apply-plan ledger.

## Context

MIMAP-174A closed the segment-map released-span local-free candidate bridge.
MIMAP-176A keeps the chain in scalar/model space and proves the next existing
owner can consume those candidate facts:

```text
segment-map released-span row
  -> local-free candidate ledger
  -> local-free apply-plan ledger
```

This prepares modeled free-list/page-state planning without mutating a real
free-list or page state.

## Scope

- Add proof app:
  `apps/hako-alloc-segment-map-local-free-apply-plan-bridge-proof/`.
- Add SSOT:
  `docs/development/current/main/design/hako-alloc-segment-map-local-free-apply-plan-bridge-ssot.md`.
- Add L2 guard:
  `tools/checks/k2_wide_hako_alloc_segment_map_local_free_apply_plan_bridge_guard.sh`.

## Stop Lines

- No real segment allocation/free execution.
- No free-list mutation.
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

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_apply_plan_bridge_guard.sh
bash tools/checks/run_proof_app.sh --only MIMAP-176A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIMAP-177A post-segment-map-local-free-apply-plan-bridge row selection
```
