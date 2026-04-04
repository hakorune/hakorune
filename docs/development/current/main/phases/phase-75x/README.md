---
Status: Landed
Date: 2026-04-04
Scope: thin the remaining top-level selfhost alias surface after folder split and blocker follow-up lanes landed.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-74x/README.md
---

# Phase 75x: Selfhost Top-Level Alias Canonicalization

## Goal

- reduce the remaining top-level `tools/selfhost/*` alias pressure after the folder split
- keep `mainline / proof / compat / lib` as the canonical tree reading
- leave only the minimum front-door wrappers at the top level

## Big Tasks

1. `75xA1` alias inventory lock
2. `75xA2` canonical path / keep-now freeze
3. `75xB1` selfhost top-level alias caller cleanup
4. `75xB2` front-door wrapper stop-line freeze
5. `75xD1` proof / closeout

## Current Read

- current front:
  - `phase-75x landed`
