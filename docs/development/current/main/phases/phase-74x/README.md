---
Status: Landed
Date: 2026-04-04
Scope: choose the next source lane after phase-73x emit_mir_mainline blocker follow-up landed.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-73x/README.md
---

# Phase 74x: Next Source Lane Selection

## Goal

- choose the next source lane after `73x`
- keep the current read stable:
  - `emit_mir_mainline` focused blocker is landed
  - stage1/selfhost mainline stays green
  - next progress should come from a fresh source lane, not from extending blocker-only follow-up

## Big Tasks

1. `74xA1` successor lane inventory lock
2. `74xA2` candidate lane ranking
3. `74xB1` successor lane decision
4. `74xD1` proof / closeout

## Current Read

- `74xA1` landed:
  - post-73x cleanup candidates are re-ranked from tree shape, not blocker follow-up
  - strongest remaining pressure is top-level alias/facade drift, especially under `tools/selfhost/`
- `74xA2` landed:
  - ranking is fixed as `75x -> 76x -> 77x`
- `74xB1` landed:
  - next lane is `phase-75x selfhost top-level alias canonicalization`
- `74xD1` landed:
  - phase-74x closes cleanly and hands off to alias thinning work
