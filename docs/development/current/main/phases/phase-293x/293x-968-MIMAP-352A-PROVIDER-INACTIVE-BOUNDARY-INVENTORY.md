# 293x-968 MIMAP-352A Provider Inactive Boundary Inventory

Status: landed
Date: 2026-05-21

## Decision

Record the provider/host integration boundary as explicitly inactive after the
worker/TLS pilot. This row consumes an accepted `HakoAllocWorkerTlsPilotReport`
and publishes scalar facts that provider activation, host allocator
replacement, hooks, `#[global_allocator]`, and backend owner-name matchers are
still closed.

## Scope

- Add `HakoAllocProviderInactiveBoundaryInventory`.
- Add a manifest-backed proof app.
- Add an L2 guard that checks static drift, VM proof output, MIR JSON shape,
  and route preflight.
- Keep the provider ladder inactive.

## Stop Lines

- No provider activation.
- No host allocator replacement.
- No hooks or `#[global_allocator]`.
- No backend `.inc` matcher by app, box, owner, or row name.
- No source-level worker-local or concurrency surface.
- No release/recycle execution.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_provider_inactive_boundary_inventory_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-352A --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed. MIMAP-353A is selected as the next row-selection card after the
provider inactive boundary inventory.
