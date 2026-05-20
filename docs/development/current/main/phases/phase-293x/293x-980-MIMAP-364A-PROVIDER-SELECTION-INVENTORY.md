# 293x-980 MIMAP-364A Provider Selection Inventory

Status: landed
Date: 2026-05-21

## Decision

Add provider selection inventory after provider readiness preflight while
keeping provider activation closed.

## Scope

- Add `HakoAllocProviderSelectionInventory`.
- Add a manifest-backed proof app.
- Add an L2 guard for static drift, VM proof output, MIR JSON shape, and route
  preflight.
- Keep provider activation and host-facing replacement/hook behavior closed.

## Stop Lines

- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No backend `.inc` matcher by app, box, owner, or row name.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_provider_selection_inventory_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-364A --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed. MIMAP-365A is selected as the next row-selection card.
