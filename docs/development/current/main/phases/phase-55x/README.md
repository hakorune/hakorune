---
Status: Active
Date: 2026-04-04
Scope: retire the last route/default/help surfaces that still make rust-vm look like a selectable live owner path.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-54x/README.md
  - docs/development/current/main/phases/phase-54x/54x-90-next-source-lane-selection-ssot.md
  - docs/development/current/main/phases/phase-54x/54x-91-task-board.md
---

# Phase 55x: Rust-VM Route-Surface Retirement Prep

## Goal

- remove the last route/default/help surfaces that still expose rust-vm as a selectable live path
- keep proof/compat/reference surfaces explicit without pruning them yet
- prepare `phase-56x` by shrinking exposure before shrinking keep-now payloads

## Plain Reading

- `phase-54x` selected this lane because route/default/help surfaces still widen rust-vm visibility.
- this lane is not the keep-pruning wave and not the delete-ready wave.
- `vm-hako` stays reference/conformance only and remains outside rust-vm retirement.

## Focus Surfaces

- `src/cli/args.rs`
- `src/runner/dispatch.rs`
- `src/runner/route_orchestrator.rs`
- `tools/selfhost/lib/selfhost_run_routes.sh`
- `tools/selfhost/run.sh`

## Success Conditions

- backend/default/help text no longer presents rust-vm as a day-to-day default
- selfhost route surfaces keep `stage-a` explicit compat-only and non-growing
- explicit router seams still work without widening the default route set
- `cargo check --bin hakorune` and `git diff --check` stay green

## Failure Patterns

- changing proof/compat payloads before route surfaces are retired
- reopening `--backend vm` or `vm-hako` as daily help/default affordances
- mixing vm-hako reference work into rust-vm route retirement

## Big Tasks

1. lock route-surface inventory and freeze exposure rules
   - `55xA1` route-surface inventory lock
   - `55xA2` backend/default/help exposure freeze
2. retire live route/default/help exposure
   - `55xB1` cli/backend affordance cleanup
   - `55xB2` selfhost route-surface cleanup
   - `55xC1` dispatch/orchestrator explicit keep narrowing
3. prove and close
   - `55xD1` proof / closeout
