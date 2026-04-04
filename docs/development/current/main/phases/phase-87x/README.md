---
Status: Landed
Date: 2026-04-04
Scope: rerun the deferred embedded snapshot / wrapper repoint seam after `84x` and `86x` landed; phase is now landed and handed off.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-86x/README.md
---

# Phase 87x: Embedded Snapshot / Wrapper Repoint Rerun

## Goal

- revisit snapshot-coupled wrapper pressure deferred by `84x`
- check whether embedded Stage1 snapshot paths can follow canonical `entry/*` ownership
- keep top-level `.hako` wrappers as explicit keep surfaces unless evidence changes

## Big Tasks

1. `87xA1` snapshot/wrapper inventory lock
2. `87xA2` target split / stop-line freeze
3. `87xB1` focused repoint rerun
4. `87xC1` proof refresh
5. `87xD1` closeout

## Current Read

- handoff complete
- landed result:
  - `embedded_stage1_modules_snapshot.json` now carries canonical `facade/runner_facade.hako`
  - `embedded_stage1_modules_snapshot.json` now carries canonical `entry/*` stubs
  - one repo-internal smoke caller was repointed to canonical `entry/launcher_native_entry.hako`
