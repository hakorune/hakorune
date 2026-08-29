---
Status: Active
Date: 2026-08-26
Scope: one-screen current lane and parked-resume map.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
---

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

1. `docs/development/current/main/CURRENT_STATE.toml`
2. the file named by `latest_workstream_card`
3. the file named by `latest_card_path`
4. the file named by `method_anchor`
5. `docs/development/current/main/design/current-docs-update-policy-ssot.md`

## Immediate Sequence

1. Run `bash tools/checks/current_state_pointer_guard.sh`.
2. Read `work_mode` before choosing an action.
3. When `work_mode = "design_stop"`, perform only the selected census/design
   row and stop before code, fixtures, caller switch, or old-path deletion.
4. When `work_mode = "fast"`, implement only the named bounded production
   cell, switch its caller, delete its selected old edge, and prove parity.
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
