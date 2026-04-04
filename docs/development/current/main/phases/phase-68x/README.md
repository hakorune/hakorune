---
Status: Active
Date: 2026-04-04
Scope: recut `lang/src/runner` so authority / compat / facade / entry reading is visible in the tree.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-67x/README.md
  - docs/development/current/main/phases/phase-67x/67x-90-selfhost-folder-split-ssot.md
---

# Phase 68x: `.hako` Runner Authority/Compat/Facade Recut

## Goal

- turn the partial `.hako` runner split into explicit folder lanes
- make `authority`, `compat`, `facade`, and `entry` reading obvious from paths
- keep the phase tree-first like `67x`, not prose-first

## Big Tasks

1. `68xA1` runner folder inventory lock
2. `68xA2` target layout ranking
3. `68xB1` facade/entry split
4. `68xB2` authority/compat split
5. `68xC1` alias/readme cleanup
6. `68xD1` proof / closeout

## Current Read

- `67x` has landed and split `tools/selfhost/` into folder lanes
- current front:
  - `68xB1 facade/entry split`
- current intent:
  - `lang/src/runner` should stop relying on file-name folklore for authority vs compat reading
  - entry/facade stubs should become obvious from folder placement
  - rust runner recut stays downstream of this `.hako` recut
