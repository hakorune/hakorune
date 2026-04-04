---
Status: Landed
Date: 2026-04-05
Scope: review and freeze policy for top-level `.hako` wrappers after the runner/selfhost recut; phase is now landed and handed off.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-90x/README.md
  - lang/src/runner/README.md
---

# Phase 91x: Top-Level .hako Wrapper Policy Review

## Goal

- keep top-level `.hako` wrappers as explicit public/front-door keeps
- preserve canonical homes under `compat/`, `facade/`, and `entry/`
- avoid reclassifying thin wrappers as archive/delete targets

## Big Tasks

1. `91xA1` top-level wrapper inventory lock
2. `91xA2` thin keep / archive boundary freeze
3. `91xB1` wrapper policy cleanup
4. `91xC1` proof refresh
5. `91xD1` closeout

## Current Read

- handoff complete
- landed result:
  - top-level wrapper inventory and policy freeze are both fixed
  - top-level wrappers remain thin public/front-door keeps
  - canonical homes stay under `compat/`, `facade/`, and `entry/`
- next corridor:
  - `phase-92x selfhost proof/compat caller rerun`
