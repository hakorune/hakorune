---
Status: Active
Date: 2026-04-04
Scope: inventory the remaining rust-vm / vm-gated source and helper surfaces, classify proof-only keep / compat keep / archive-later / delete-ready, and archive only drained surfaces.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-50x/50x-90-rust-vm-source-archive-cleanup-ssot.md
  - docs/development/current/main/phases/phase-50x/50x-91-task-board.md
---

# Phase 50x: Rust VM Source / Archive Cleanup

## Goal

- inventory the remaining live `--backend vm` / rust-vm source surfaces
- keep proof-only and compat routes explicit and non-growing
- move archive-ready docs/helpers out of the live current surface
- preserve `cargo check --bin hakorune` and `git diff --check`

## Plain Reading

- phase-49x finished wording cleanup and handoff.
- phase-50x now deals with actual source/helper/archive hygiene.
- PyVM is already historical/direct-only and is not a day-to-day blocker.
- the remaining live surfaces are mostly proof-only or compat keep; the job is to separate them from archive-ready leftovers.

## Success Conditions

- current docs no longer imply daily ownership from rust-vm/PyVM routes
- proof-only gates and compat keeps remain explicit
- archive-ready surfaces can be moved without reintroducing live `--backend vm` defaults
- source/helper edits stay green on `cargo check --bin hakorune`

## Failure Patterns

- deleting proof/compat keeps before replacement or classification
- reintroducing default-vm routes while cleaning source/helper commentary
- treating historical PyVM tooling as day-to-day active ownership

## Big Tasks

1. `50xA` inventory and classification
   - `50xA1` residual rust-vm surface inventory lock (landed)
   - `50xA2` proof-only / compat keep classification (active)
2. `50xB` stale helper cleanup
   - `50xB1` smoke/helper stale-route cleanup
   - `50xB2` route-comment stale wording cleanup
3. `50xC` archive-ready source/docs move
   - `50xC1` archive-ready docs/examples move
   - `50xC2` historical PyVM / legacy wrapper archival sweep
4. `50xD` proof / closeout
   - `50xD1` proof / closeout

## Boundaries

- proof-only gates stay proof-only
- compat keeps remain explicit and non-growing
- historical PyVM tooling stays historical/direct-only
- delete only after caller drain
