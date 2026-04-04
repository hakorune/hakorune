---
Status: Active
Date: 2026-04-04
Scope: choose the next source lane after phase-73x landed.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-73x/README.md
---

# 74x-90 Next Source Lane Selection SSOT

## Intent

- keep the post-73x read stable
- rank the next source lane without reopening the closed `emit_mir_mainline` blocker

## Current Read

- landed:
  - `phase-73x emit_mir_mainline blocker follow-up`
- now:
  - `74xA1 successor lane inventory lock`
