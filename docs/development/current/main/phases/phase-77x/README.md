---
Status: Active
Date: 2026-04-04
Scope: thin the remaining top-level owner pressure in lang/src/runner after wrapper canonicalization settled.
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

- current front:
  - `77xA2 target split / stop-line freeze`
