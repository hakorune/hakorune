---
Status: Landed
Date: 2026-04-27
Scope: Refresh stale phase task-order baseline after row-prune and route-boundary cleanup
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-255-post-birth-cleanup-task-order-card.md
  - docs/development/current/main/phases/phase-291x/291x-487-map-lookup-fusion-route-field-closeout-card.md
---

# 291x-488: Current Task Order Baseline Refresh

## Result

Refreshed stale phase-291x docs after the `.inc` row-prune sequence and the
route-boundary cleanup burst.

- `291x-255` now explicitly records its `classifiers=14 rows=14` baseline as a
  historical baseline.
- Current phase docs now point at `classifiers=0 rows=0`.
- README navigation points to this current checkpoint instead of the older
  `291x-480` next-lane card.
- Restart/current mirrors point at `291x-488` while keeping the active blocker
  as `phase-291x next lane selection pending`.

## Current Baseline

```text
classifiers=0
rows=0
```

The `.inc` method/box classifier prune order from `291x-255` is closed. Future
phase-291x work should treat new work as BoxShape cleanup unless a new
BoxCount/card explicitly reopens a feature or proof lane.

## Next

Select the next phase-291x compiler-cleanliness lane as a separate BoxShape
card. Do not use stale `.inc` row-prune tasks as the active task list.
