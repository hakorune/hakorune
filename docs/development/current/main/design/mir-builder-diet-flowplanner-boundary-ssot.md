# MIR Builder Diet / FlowPlanner Boundary SSOT

Status: SSOT
Date: 2026-05-15
Scope: temporary BoxShape cleanup sidecar before returning to `MIMAP-021C`.

Related:
- `src/mir/README.md`
- `src/mir/builder/README.md`
- `src/mir/builder/control_flow/plan/ARCHITECTURE.md`
- `src/mir/builder/control_flow/plan/REGISTRY.md`
- `docs/development/current/main/design/mir-crate-split-prep-ssot.md`
- `docs/development/current/main/phases/phase-293x/293x-mir-builder-diet-taskboard.md`

## Problem

`src/mir/builder/control_flow` has grown into more than MIR builder core:

```text
src                         498,142 lines
src/mir                     281,056 lines
src/mir/builder             113,185 lines
src/mir/builder/control_flow  92,132 lines
src/mir/builder/control_flow/plan 61,942 lines
```

The size is not automatically bad. Hakorune intentionally keeps source syntax
small and pushes control-flow proof, JoinIR, and CorePlan contracts into the
compiler. The problem is that the physical path still makes these subsystems
look like builder core.

## Decision

Treat this sidecar as a BoxShape cleanup, not a behavior row.

The cleanup goal is to make ownership visible before more mimalloc allocator
rows add pressure:

```text
MirBuilder core:
  AST -> canonical MIR emission
  ValueId / BlockId issuance
  lexical scope / bindings / locals
  source provenance / diagnostics
  simple lowering and actual MIR block assembly

FlowPlanner:
  control-flow shape facts
  recipe contracts
  CorePlan skeletons / features
  planner_required fail-fast boundaries
  plan lowering contracts

JoinIR:
  normalized control-flow layer, ownership analysis, merge/bridge fence

SemanticPlans:
  MIR semantic facts, layout plans, placement plans, lowering routes,
  experimental seed routes
```

Physical moves are not part of the first cleanup row. The first row only fixes
the docs and public-entry contract so future crate split remains mechanical.

## Keep In Builder

- AST node dispatch into MIR emission.
- `MirBuilder::next_value_id()` and related ID issuance.
- Scope, binding, local, and current-block state through the existing Context
  owners.
- Source span / diagnostic provenance.
- Calling one documented FlowPlanner entry instead of reaching into plan
  internals.

## Move Conceptually Out

- `control_flow/plan` as FlowPlanner subsystem, even while it physically lives
  under builder.
- CorePlan skeleton / feature / recipe vocabulary.
- JoinIR ownership / merge / VM bridge semantics.
- MIR semantic metadata refresh and layout/placement/lowering-route plans.
- Backend route acceptance policy.

## Hard Rules

- No behavior changes in `MIRBUILDER-DIET-001`.
- No physical crate split in this sidecar unless a later row explicitly proves a
  pure package seam.
- No new `loop_*_v0` one-shape box without a `retire_when` / promotion path.
- Do not add new legacy normalizer branches after the second similar case; move
  the shape into skeleton + feature composition.
- `strict/dev + planner_required` must fail-fast; no silent fallback.
- Release default stays unchanged.
- Do not mix this BoxShape cleanup with `MIMAP-*` allocator behavior rows.

## Cleanup Rows

| Row | Status | Purpose |
| --- | --- | --- |
| `MIRBUILDER-DIET-001` | landed | Open the sidecar and pin this boundary SSOT. |
| `FLOWPLANNER-ENTRY-001` | landed | Inventory builder -> FlowPlanner public entries and document rejected bypasses. |
| `FLOWPLANNER-V0-001` | landed | Add retire/promote rules for active `loop_*_v0` boxes. |
| `MIR-SEMANTIC-PLANS-001` | ready | Classify top-level MIR plan/route/seed owners without moving files. |
| `JOINIR-FENCE-001` | parked | Tighten JoinIR merge/bridge boundary after FlowPlanner entry is stable. |

## Initial Inventory Notes

FlowPlanner entry pressure:

- main loop path is `MirBuilder::cf_loop` -> `joinir::route_entry::router::route_loop`.
- intended planner path is `route_loop` -> `try_build_outcome` -> recipe-first
  registry -> `RecipeComposer` -> verifier -> plan lowerer.
- `plan/mod.rs` currently exposes too much as builder-visible surface.
- `control_flow/lower/planner_compat.rs` is the best current facade candidate.
- `return_stmt.rs` directly adopts match-return CorePlan and creates a
  `LoopRouteContext`; this needs a named non-loop CorePlan adoption boundary,
  not another hidden builder bypass.

SemanticPlans classification pressure:

- keep `LayoutPlans`, `PlacementPlans`, `LoweringRoutes`,
  `ExperimentalSeedRoutes`, and `SemanticFacts/Contracts` distinct.
- "plan" means compiler-owned candidate/selection; "route" means backend/VM
  consumable contract.
- do not physically move top-level `src/mir/*_plan*` files in the first
  classification row.

## Return Condition

Return to `MIMAP-021C` when:

- `src/mir/builder/README.md` clearly separates builder core from FlowPlanner.
- `src/mir/builder/control_flow/plan/REGISTRY.md` names the public entry and
  rejected bypasses.
- top-level MIR plan/route/seed surfaces have a classification owner or a
  follow-up card.
- `bash tools/checks/current_state_pointer_guard.sh` is green.
