---
Status: Landed
Date: 2026-04-04
Scope: thin remaining top-level `.hako` wrapper and source contract pressure after selfhost top-level keeps are frozen; phase is now landed and handed off.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-83x/README.md
---

# Phase 84x: Runner Wrapper/Source Contract Thinning

## Goal

- reduce the remaining top-level `.hako` wrapper pressure
- clarify which wrappers stay as interface stubs and which contracts belong to canonical owner paths
- keep `emit_mir_mainline` and `stage1_mainline_smoke` green while thinning

## Big Tasks

1. `84xA1` wrapper/source inventory lock
2. `84xA2` target split / stop-line freeze
3. `84xB1` wrapper/source thinning
4. `84xC1` proof refresh
5. `84xD1` closeout

## Current Read

- handoff complete
- landed result:
  - Stage1 build/default contracts point at canonical `lang/src/runner/entry/*` stubs
  - top-level `.hako` wrappers remain explicit keep surfaces
