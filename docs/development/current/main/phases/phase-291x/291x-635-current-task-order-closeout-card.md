---
Status: Landed
Date: 2026-04-28
Scope: rebaseline current task order after plan-side compat queue closeout
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-575-plan-compat-residue-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-620-broad-plan-facts-migration-boundary-card.md
---

# 291x-635: Current Task Order Closeout

## Goal

Close stale current/restart pointers that still described the
`291x-575` plan-side compat/facade cleanup queue as active.

This is docs-only BoxShape cleanup. It does not change code, smoke selection,
accepted shapes, planner behavior, or ownership boundaries.

## Evidence

The current mirrors still said to continue the plan-side compat queue and named
`291x-576` as the latest checkpoint.

The actual queue state is closed:

- `291x-575` through `291x-586` are marked `done` in `todos.db`.
- The ordered compat/facade cleanup continued through follow-up cards and the
  latest code cleanup card is `291x-634`.
- `291x-620` explicitly closed broad `plan/facts` migration as a boundary
  decision, not as work to keep doing inside the old queue.

## Decision

The active blocker token is now:

```text
phase-291x next compiler-cleanliness lane selection pending
```

Next work is lane selection, not continuing the old queue. Broad `plan/facts`
or `lower::planner_compat` ownership work can reopen only as a new
family-sized BoxShape lane with its own SSOT and acceptance boundary.

## Result

- Updated `CURRENT_STATE.toml` latest-card fields and active blocker token.
- Updated `CURRENT_TASK.md`, restart, now, and phase README mirrors.
- Added this closeout card as the current task-order source.

## Verification

```bash
sqlite3 todos.db "select id,status,description from todos where id like '291x-57%' or id like '291x-58%' order by id;"
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
