---
Status: Active
Date: 2026-04-04
Scope: select the next source lane after runner top-level pressure thinning lands.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-77x/README.md
---

# Phase 78x: Next Source Lane Selection

## Goal

- pick the next source lane after `77x` is proof-closed
- keep the known launcher probe red as a residual blocker, not as the current lane owner
- turn the remaining pressure into one selected follow-up lane

## Big Tasks

1. `78xA1` successor lane inventory lock
2. `78xA2` candidate lane ranking
3. `78xB1` successor lane decision
4. `78xD1` proof / closeout

## Current Read

- current front:
  - `78xA1 successor lane inventory lock`
- known residual blocker:
  - `launcher.hako emit_mir_mainline probe still red`
