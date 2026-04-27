---
Status: Landed
Date: 2026-04-28
Scope: inventory remaining plan-side compat/facade seams and fix next cleanup queue
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-291x/README.md
  - todos.db
---

# 291x-575: Plan Compat Residue Inventory

## Goal

Replace the vague `next lane selection pending` state with a concrete,
ordered plan-side compat/facade cleanup queue.

This is a docs/task inventory card only. It does not change compiler behavior.

## Worker Inventory Summary

The read-only worker inventory found the remaining plan-side seams in three
buckets:

| Bucket | Count | Meaning |
| --- | ---: | --- |
| S | 2-3 | small wrappers or zero/low-reference facades that can be pruned in one card |
| M | 4-5 | caller migration needed before deleting the compatibility surface |
| L | 3 | boundary or design review before code movement |

## Ordered Queue

| Order | Card | Size | Target | Notes |
| ---: | --- | --- | --- | --- |
| 1 | `291x-576` | S | `plan::coreloop_body_contract` wrapper | migrate roughly five users to `verify::coreloop_body_contract`, then delete wrapper |
| 2 | `291x-577` | S | zero/unused `plan/facts` wrappers | start with `expr_value` / `block_policies` if still zero-use |
| 3 | `291x-578` | M | small `plan::facts` owner-path migration | `no_exit_block`, `stmt_view`, `expr_bool`; split if churn grows |
| 4 | `291x-579` | M | `plan::extractors::common_helpers` migration | migrate by function family; do not bulk-rewrite blindly |
| 5 | `291x-580` | M | `plan::canon::cond_block_view` migration | wide but mechanical; keep as its own card |
| 6 | `291x-581` | S/M | plan-side SSA exit-binding wrappers | migrate direct callers to `ssa` owner paths |
| 7 | `291x-582` | M | `recipes::composer_compat` review/prune | inspect re-export users before deletion |
| 8 | `291x-583` | M/L | `lower::planner_compat` inventory | likely needs implementation move before deleting compat facade |
| 9 | `291x-584` | L | `plan::edgecfg_facade` boundary review | do not delete blindly; this is a Plan boundary seam |
| 10 | `291x-585` | L | legacy v0 family boundary inventory | `loop_*_v0` owns live behavior; inventory before code movement |
| 11 | `291x-586` | L | broad `plan/facts` owner migration plan | about 175 `plan::facts` references; stage after small wrappers |

## Current Blocker Token

```text
phase-291x plan-side compat surface prune queue active
```

## Boundaries

- Keep this queue BoxShape-only.
- Do not add accepted control-flow shapes.
- Do not mix hot lowering, CoreMethodContract/CoreOp migration, or Stage-B
  adapter thinning into these cards.
- If a candidate has broad live behavior, land an inventory/review card before
  moving code.

## Acceptance

- `CURRENT_STATE.toml` points at this card.
- Thin restart mirrors contain the new blocker token.
- `todos.db` contains the ordered `291x-576` through `291x-586` queue.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `git diff --check` passes.

## Result

- Fixed the next cleanup lane as plan-side compat/facade seam pruning.
- Updated the root/current/restart mirrors because the blocker token changed.
- Replaced the stale `291x-447` pending todo with the concrete ordered queue.

## Verification

```bash
sqlite3 todos.db "SELECT id, status, description FROM todos WHERE id LIKE '291x-57%' OR id LIKE '291x-58%' ORDER BY id;"
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
