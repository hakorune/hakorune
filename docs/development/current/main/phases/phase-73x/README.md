---
Status: Active
Date: 2026-04-04
Scope: follow up the focused `emit_mir_mainline` blocker after facade-thinning lanes closed as no-op.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-65x/README.md
  - docs/development/current/main/phases/phase-72x/README.md
---

# Phase 73x: emit_mir_mainline Blocker Follow-Up

## Goal

- reproduce and narrow the tracked `emit_mir_mainline` parse red
- keep stage1/selfhost mainline green while fixing the focused blocker
- prefer a narrow source fix over further facade-only no-op lanes

## Big Tasks

1. `73xA1` blocker evidence lock
2. `73xA2` target fix ranking
3. `73xB1` focused source fix
4. `73xC1` proof bundle refresh
5. `73xD1` proof / closeout

## Current Read

- `73xA1` landed:
  - focused repro is confirmed for both `stage1_cli_env.hako` and `stage1_cli.hako`
  - merged origin still points at `lang/src/compiler/build/build_box.hako:4`
  - `tools/selfhost/mainline/stage1_mainline_smoke.sh` stays green
- `73xA2` landed:
  - reduced file-context repro still points at merged `BuildBox`
  - first fix target is the selfhost-first merge/parser seam around `build_box`
- `73xB1` active:
  - `build_box` parse seam is fixed
  - `stage1_cli_env.hako` focused probe is green again
  - remaining red is narrowed to top-level `stage1_cli.hako` facade lowerability under selfhost-first `emit_mir_mainline`
- current front:
  - `73xB1 focused source fix`
