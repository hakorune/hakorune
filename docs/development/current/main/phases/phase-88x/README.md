---
Status: Active
Date: 2026-04-04
Scope: rerun archive/delete-ready inventory after the latest wrapper and snapshot repoints.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-87x/README.md
---

# Phase 88x: Archive/Deletion Rerun

## Goal

- check whether the latest wrapper/snapshot repoints created true archive-ready residue
- separate explicit keep surfaces from real delete-ready/archive-ready payload
- avoid deleting public/front-door keep wrappers by accident

## Big Tasks

1. `88xA1` archive-ready inventory lock
2. `88xA2` keep/archive/delete-ready classification
3. `88xB1` focused archive/deletion sweep
4. `88xC1` proof refresh
5. `88xD1` closeout

## Current Read

- current front:
  - `88xA1 archive-ready inventory lock`
- target seam:
  - top-level wrapper aliases after `87x`
  - stale archive candidates created by canonical repoints
