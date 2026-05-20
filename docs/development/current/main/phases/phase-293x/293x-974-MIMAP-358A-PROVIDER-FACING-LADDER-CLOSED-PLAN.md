# 293x-974 MIMAP-358A Provider-Facing Ladder Closed Plan

Status: landed
Date: 2026-05-21

## Decision

Plan the provider-facing allocator ladder while keeping provider activation,
host allocator replacement, hooks, and `#[global_allocator]` closed. The next
behavior row should inventory provider boundary diagnostic vocabulary before
any provider readiness or selection behavior.

## Planned Order

1. Provider boundary diagnostic vocabulary inventory.
2. Provider readiness preflight with activation closed.
3. Provider selection inventory with activation closed.
4. Provider activation first-pattern row only after explicit row selection.
5. Host allocator replacement / hooks / `#[global_allocator]` as separate
   optional ladders.

## Stop Lines

- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No backend `.inc` matcher by app, box, owner, or row name.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_provider_facing_ladder_closed_plan_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed. MIMAP-359A is selected as the next row-selection card.
