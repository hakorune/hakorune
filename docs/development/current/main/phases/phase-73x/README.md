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
