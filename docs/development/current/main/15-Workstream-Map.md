---
Status: Active
Date: 2026-04-13
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
| Now | `phase-163x primitive and user-box fast path` |
| Front | `generic placement / effect landed through the first owner-transform cut -> next design lane is semantic simplification bundle` |
| Guardrail | `phase-137x` string corridor / `kilo_micro_substring_views_only` |
| Blocker | `semantic simplification bundle is active; latest cut folds copied-constant Branch, constant Compare, and empty trampoline jumps before CFG merge` |
| Next | `semantic simplification bundle` |
| After Next | `memory-effect layer` |

## Current Read

- design owners:
  - implementation lane: `docs/development/current/main/phases/phase-163x/README.md`
  - next layer landing: `docs/development/current/main/phases/phase-227x/README.md`
  - roadmap SSOT: `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
  - sibling string guardrail: `docs/development/current/main/phases/phase-137x/README.md`
- recent landed:
  - `phase-242x`: `task_scope` / `TaskGroupBox` structured-concurrency vocabulary alignment
  - `semantic simplification bundle`: copied-constant Branch fold and constant Compare fold before CFG merge
  - `phase-227x`: semantic simplification owner seam
  - `phase-226x`: placement-effect string scheduling owner cut

## Immediate Sequence

1. `semantic simplification bundle`
2. `memory-effect layer`
3. `escape / barrier -> LLVM attrs`

## Parked Corridor

- `phase-96x vm_hako LLVM acceptance cutover`
- monitor-policy decision for the frozen `vm-hako-core` pack remains the only backlog there
