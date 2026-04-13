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
| Now | `phase-277x optimization lane closeout judgment` |
| Front | `optimization lane closeout judgment` |
| Guardrail | `phase-137x` string corridor / `kilo_micro_substring_views_only` |
| Blocker | `IPO lane is functionally landed; the next queue is closeout judgment` |
| Next | `optimization lane closeout judgment` |
| After Next | `post-optimization roadmap refresh` |

## Current Read

  - design owners:
  - implementation lane: `docs/development/current/main/phases/phase-277x/README.md`
  - next layer landing: `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
  - roadmap SSOT: `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
  - sibling string guardrail: `docs/development/current/main/phases/phase-137x/README.md`
  - concurrency manual owner: `docs/reference/concurrency/semantics.md`
  - concurrency runtime-plan owner: `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md`

## Immediate Sequence

1. `optimization lane closeout judgment`
2. `post-optimization roadmap refresh`

## Parked Corridor

- `phase-96x vm_hako LLVM acceptance cutover`
- monitor-policy decision for the frozen `vm-hako-core` pack remains the only backlog there
