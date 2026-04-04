---
Status: Landed
Date: 2026-04-05
Scope: thin current/design docs that still described old wrapper/current surfaces too noisily after the runner/selfhost recuts; phase is now landed and handed off.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-91x/README.md
---

# Phase 90x: Current-Doc/Design Stale Surface Hygiene

## Goal

- thin current/design docs that over-described old wrapper/current surfaces
- keep root/current mirrors thin while preserving explicit keep stop-lines
- avoid reopening archive/delete decisions that `88x` already proved are still no-op

## Big Tasks

1. `90xA1` stale surface inventory lock
2. `90xA2` target split / stop-line freeze
3. `90xB1` current/design stale surface cleanup
4. `90xC1` proof refresh
5. `90xD1` closeout

## Current Read

- handoff complete
- landed result:
  - current/design stale surface wording is thinned
  - root/current mirrors remain thin and consistent
- next corridor:
  - `phase-91x top-level .hako wrapper policy review`
  - `phase-92x selfhost proof/compat caller rerun`
