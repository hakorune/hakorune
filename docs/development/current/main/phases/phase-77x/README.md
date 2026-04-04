---
Status: Landed
Date: 2026-04-04
Scope: thin the remaining top-level owner pressure in lang/src/runner after wrapper canonicalization settled; phase is now landed and handed off.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-76x/README.md
---

# Phase 77x: Runner Top-Level Pressure Thinning

## Goal

- reduce remaining top-level owner pressure in `lang/src/runner`
- keep thin wrappers as wrappers and move body pressure behind canonical clusters
- avoid reopening already-settled wrapper canonicalization work

## Big Tasks

1. `77xA1` runner top-level owner inventory lock
2. `77xA2` target split / stop-line freeze
3. `77xB1` `launcher.hako` body thinning
4. `77xB2` `stage1_cli_env.hako` authority thinning
5. `77xD1` proof / closeout

## Current Read

- handoff complete
- next lane: `phase-78x next source lane selection`
- current front: `78xA1 successor lane inventory lock`
