---
Status: Active
Date: 2026-04-22
Scope: current mainline / next lane / parked corridor の one-screen map。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
---

# Workstream Map

## Current Lane

| Item | State |
| --- | --- |
| Now | `phase-290x ArrayBox surface canonicalization` |
| Front | `catalog/invoke seam landed -> stable smoke landed -> app follow-up` |
| Guardrail | `phase-137x observe-only perf reopen rule` |
| Blocker | `ArrayBox truth is split across runtime surface, dispatch, std sugar, and smoke` |
| Next | `resume kilo editor slices (backspace merge / undo execution)` |
| After Next | `defer slice return-type union receiver cleanup unless app work needs it` |

## Current Read

  - design owners:
  - implementation lane: `docs/development/current/main/phases/phase-290x/README.md`
  - phase brief: `docs/development/current/main/phases/phase-290x/290x-90-arraybox-surface-canonicalization-design-brief.md`
  - taskboard: `docs/development/current/main/phases/phase-290x/290x-91-arraybox-surface-task-board.md`
  - inventory: `docs/development/current/main/phases/phase-290x/290x-92-arraybox-surface-inventory-ledger.md`
  - sibling string guardrail: `docs/development/current/main/phases/phase-137x/README.md`

## Immediate Sequence

1. `phase-290x docs-first ArrayBox surface lock`
2. `phase-290x implementation seam` (landed)
3. `phase-290x stable smoke` (landed)
4. `return to kilo editor feature slices`

## Parked Corridor

- `phase-96x vm_hako LLVM acceptance cutover`
- monitor-policy decision for the frozen `vm-hako-core` pack remains the only backlog there
