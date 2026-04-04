---
Status: Active
Date: 2026-04-04
Scope: thin the remaining top-level selfhost facades after the folder split landed.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-67x/README.md
  - docs/development/current/main/phases/phase-71x/README.md
---

# Phase 72x: Selfhost Top-Level Facade Thinning

## Goal

- reduce top-level `tools/selfhost/*.sh` pressure after the folder split
- keep `mainline / proof / compat / lib` as the canonical tree reading
- leave only thin facades at the top level where they still help entry/readability

## Big Tasks

1. `72xA1` top-level facade inventory lock
2. `72xA2` keep-vs-thin ranking
3. `72xB1` facade thinning wave
4. `72xC1` current pointer cleanup
5. `72xD1` proof / closeout

## Current Read

- `72xA1` landed:
  - top-level `run.sh`, `selfhost_build.sh`, and `build_stage1.sh` stay front-door keep
  - canonical-backed smoke/compat/proof wrappers are the first thinning targets
- `72xA2` landed:
  - first-wave thinning stayed focused on canonical-backed wrappers
- `72xB1` landed:
  - those wrappers were already thin exec facades, so the wave closed as a no-op
- `72xC1` landed:
  - current pointers match that no-op result
  - adjacent top-level `.hako` wrappers are already thin too
- current front:
  - `72xD1 proof / closeout`
