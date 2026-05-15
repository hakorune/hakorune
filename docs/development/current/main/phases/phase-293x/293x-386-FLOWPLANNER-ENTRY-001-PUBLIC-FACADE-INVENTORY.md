# 293x-386 FLOWPLANNER-ENTRY-001 Public Facade Inventory

Status: ready
Date: 2026-05-15

## Decision

`src/mir/builder/control_flow/plan` is the FlowPlanner subsystem, not builder
core. This row inventories and documents the public entry from builder into
FlowPlanner, plus rejected bypasses.

## Scope

- Update `src/mir/builder/README.md` to name FlowPlanner as conceptually
  outside builder core.
- Update `src/mir/builder/control_flow/plan/REGISTRY.md` with the public entry
  and rejected bypass list.
- Document accepted temporary exceptions:
  - router direct helper imports
  - registry handler imports
  - top-level facts/recipes imports
  - `return_stmt.rs` non-loop CorePlan adoption
- Keep the physical path unchanged.
- Do not alter routing behavior.

## Stop Lines

- No behavior change.
- No physical move.
- No new accepted loop/control-flow shape.
- No new `loop_*_v0` box.
- No planner fallback change.

## Required Evidence

```text
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
