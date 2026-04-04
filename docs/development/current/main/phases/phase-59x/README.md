---
Status: Landed
Date: 2026-04-04
Scope: continue retiring rust-vm route/default/help surfaces after phase-58x selected this lane as the highest-leverage successor.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-58x/README.md
  - docs/development/current/main/phases/phase-58x/58x-90-next-source-lane-selection-ssot.md
  - docs/development/current/main/phases/phase-58x/58x-91-task-board.md
---

# Phase 59x: Rust-VM Route-Surface Retirement Continuation

## Goal

- keep shrinking explicit rust-vm route/default/help affordances without touching the keep-now source cores
- reduce future rust-vm pressure at the CLI/selfhost/orchestrator boundary
- leave `vm-hako` reference/conformance out of scope

## Focus Surfaces

- `src/cli/args.rs`
- `src/runner/dispatch.rs`
- `src/runner/route_orchestrator.rs`
- `tools/selfhost/lib/selfhost_run_routes.sh`
- `tools/selfhost/run.sh`
- top-level help/docs/examples that still over-expose explicit VM route selection

## Success Conditions

- explicit compat/proof routes remain available but more clearly bounded
- route/default/help exposure is narrower than phase-55x
- `cargo check --bin hakorune` and `git diff --check` stay green

## Big Tasks

1. inventory and freeze the remaining route surfaces
   - `59xA1` route-surface inventory lock
   - `59xA2` route/default/help exposure freeze
2. narrow the live affordances
   - `59xB1` CLI/backend affordance narrowing
   - `59xB2` selfhost route/default narrowing
   - `59xC1` dispatch/orchestrator affordance narrowing
3. prove and close
   - `59xD1` proof / closeout

## Current Front

- `59xA1` landed
- `59xA2` landed
- `59xB1` landed
- `59xB2` landed
- `59xC1` landed
- `59xD1` landed
- next: `phase-60x proof/compat keep pruning continuation`
