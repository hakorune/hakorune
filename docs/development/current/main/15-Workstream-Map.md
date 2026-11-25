---
Status: Active
Date: 2026-11-17
Scope: one-screen current lane and parked-resume map.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/RULES.md
---

> **今の進路(メンテナ決定 2026-11-17)。** 機能を保ちながら、今ある MirBuilder を責務に分けて、要らない code と test を消す。別の builder は作らない。
> 作業の時に読むのは、次の 6 つだけ: `AGENTS.md`(または `CLAUDE.md`)、`docs/development/RULES.md`、`docs/development/current/main/CURRENT_STATE.toml`、`docs/development/current/main/design/mirbuilder-blueprint.md`(設計と、14 節の進め方)、`docs/development/current/main/investigations/mirbuilder-blueprint-work-2026-11-17.md`(今の順と、タスクの一覧)、`src/mir/README.md`。
> **この file の下の一覧は、古い案内である。作業の根拠にしない。** 「active」「SSOT」「唯一」と書いてあっても、上の 6 つと食い違う時は、上の 6 つが正しい。


# Workstream Map

## Current Lane

| Item | Source |
| --- | --- |
| Active lane | `active_lane` in `CURRENT_STATE.toml` |
| Workstream | `latest_workstream_card` in `CURRENT_STATE.toml` |
| Front | `latest_card_path` in `CURRENT_STATE.toml` |
| Blocker | `current_blocker_token` in `CURRENT_STATE.toml` |
| Scope | `latest_card_summary` in `CURRENT_STATE.toml` |
| Exact task order | file named by `latest_workstream_card` |

## Current Read

1. `docs/development/RULES.md`
2. `docs/development/current/main/CURRENT_STATE.toml`
3. the file named by `latest_card_path`
4. `docs/development/current/main/design/mirbuilder-blueprint.md` (when the work touches `src/mir`)
5. `docs/development/adr/adr-003-counted-box-default.md`, `adr-004-failure-surface-catch-and-cleanup.md`, `adr-005-file-functions-live-in-static-boxes.md`

`latest_workstream_card` and the milestone tables below are history (2026-07..10), not the current order.

## Immediate Sequence

1. Run `bash tools/checks/current_state_pointer_guard.sh`.
2. Read `work_mode` before choosing an action.
3. When `work_mode = "design_stop"`, perform only the selected census/design
   row and stop before code, fixtures, caller switch, or old-path deletion.
4. When `work_mode = "fast"`, implement only the named bounded slice. Keep
   construction closeout distinct from production cutover and old-edge
   retirement; apply the active card and RULES exit conditions.
5. When `work_mode = "closeout"`, classify evidence and synchronize the
   owning docs/guard/commit without widening the implementation.

## Milestone Projection (navigation only)

This table is a compact progress view, not a second authority or task ledger.
The active row and completion claims still come from `CURRENT_STATE.toml` and
the linked SSOTs.

| Milestone | Current state | Completion signal | Authority |
| --- | --- | --- | --- |
| MS1 Call spine | active, design stop at the current compatibility boundary | Call R6/R7, old resolver/recovery/fallback edges and `callee=None` producers are zero | `mirbuilder-final-pipeline-ssot.md` and the active Call card |
| MS2 canonical pipeline / Loop | parked sibling | canonical function finish plus the existing Loop M8--M12 and final convergence audit are accepted | `mirbuilder-final-pipeline-ssot.md` and `joinir-loop-selfhost-recipe-pipeline-ssot.md` |
| MS3 VM retirement | parked sibling, after Call R7 and post-Call integration | LLVM/AOT is the product owner, the explicit reference is temporary, and Rust VM callers reach zero before deletion | `vm-active-lane-retirement-ssot.md` |
| MS4 future selfhost artifact | parked future lane | a selfhost owner defines and proves its first artifact; it is not a prerequisite for MS1 | `selfhost-lift-boundary-and-task-order-ssot.md` |

Do not express these milestones as percentages. A milestone is complete only
when its linked owner records the production-graph delta and required
caller-zero/parity evidence.

## Parked Resume

```text
Stage-B / Ownership / Language v1 / selfhost:
  parked until CURRENT_STATE.toml explicitly reopens them
```

Optimization, allocator replacement, provider activation, and broad selfhost
authority remain parked unless `CURRENT_STATE.toml` explicitly reopens them.
