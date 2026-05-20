---
Status: SSOT
Decision: accepted
Date: 2026-05-21
Row: MIMAP-358A
Scope: provider-facing ladder plan with activation still closed.
Related:
  - docs/development/current/main/phases/phase-293x/293x-974-MIMAP-358A-PROVIDER-FACING-LADDER-CLOSED-PLAN.md
  - tools/checks/k2_wide_hako_alloc_provider_facing_ladder_closed_plan_guard.sh
---

# Hako Alloc Provider-Facing Ladder Closed Plan

## Decision

MIMAP-358A opens planning for the provider-facing allocator ladder without
activating any provider behavior.

The next provider-facing work must proceed in this order:

1. provider boundary diagnostic vocabulary inventory
2. provider readiness preflight with activation still closed
3. provider selection inventory with activation still closed
4. provider activation first-pattern row, only after explicit selection
5. host allocator replacement / hooks / `#[global_allocator]` remain separate
   optional ladders

## Layer Boundary

Provider-facing rows are not allowed to bypass the compiler route contract:

- backend must consume MIR route metadata
- no backend `.inc` owner-name matcher
- no app/box/row-name matching
- unsupported provider behavior must fail fast in the row guard or preflight

## Stop Lines

- No provider activation.
- No host allocator replacement.
- No hooks or `#[global_allocator]`.
- No backend `.inc` matcher by app, box, owner, or row name.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_provider_facing_ladder_closed_plan_guard.sh
```
