---
Status: Active
Date: 2026-04-04
Scope: split tools/selfhost into mainline / proof / compat / lib surfaces, keeping the top-level facade-only.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-66x/README.md
  - docs/development/current/main/phases/phase-66x/66x-90-next-source-lane-selection-ssot.md
---

# Phase 67x: Selfhost Folder Split

## Goal

- split `tools/selfhost/` into folder-level lanes instead of continuing prose-only cleanup
- make the common routes visible as tree shape:
  - `mainline/`
  - `proof/`
  - `compat/`
  - `lib/`
- keep top-level `tools/selfhost/` facade-only

## Big Tasks

1. `67xA1` selfhost folder inventory lock
2. `67xA2` target layout ranking
3. `67xB1` top-level selfhost split
4. `67xB2` proof/compat split
5. `67xC1` lib/alias cleanup
6. `67xD1` proof / closeout

## Current Read

- `66x` has landed and selected `phase-67x selfhost folder split`
- current front:
  - `67xC1 lib/alias cleanup`
- current intent:
  - `tools/selfhost` should stop mixing mainline / proof / compat / lib at the top level
  - runtime and proof callers should become obvious from folder placement
  - archive work stays downstream of folder separation
  - canonical script homes now move under `mainline/`, `proof/`, and `compat/` while top-level files stay thin wrappers
