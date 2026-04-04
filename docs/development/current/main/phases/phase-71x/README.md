---
Status: Active
Date: 2026-04-04
Scope: choose the next source lane after phase-70x caller-zero archive sweep closed as a no-op.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-70x/README.md
  - docs/development/current/main/phases/phase-70x/70x-90-caller-zero-archive-sweep-ssot.md
---

# Phase 71x: Next Source Lane Selection

## Goal

- choose the next source lane after `70x`
- keep the current read stable:
  - selfhost folder split is landed
  - `.hako` runner recut is landed
  - rust runner product/keep/reference recut is landed
  - caller-zero archive sweep closed as a no-op
  - next progress should come from a new source lane, not from forcing archive churn

## Big Tasks

1. `71xA1` successor lane inventory lock
2. `71xA2` candidate lane ranking
3. `71xB1` successor lane decision
4. `71xD1` proof / closeout
