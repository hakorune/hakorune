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
| Now | `phase-270x closure split env scalarization owner seam` |
| Front | `closure split -> env scalarization owner seam` |
| Guardrail | `phase-137x` string corridor / `kilo_micro_substring_views_only` |
| Blocker | `numeric loop / SIMD is closed out; the next queue is closure split env scalarization work` |
| Next | `closure split` |
| After Next | `IPO / build-time optimization` |

## Current Read

  - design owners:
  - implementation lane: `docs/development/current/main/phases/phase-270x/README.md`
  - next layer landing: `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
  - roadmap SSOT: `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
  - sibling string guardrail: `docs/development/current/main/phases/phase-137x/README.md`
  - concurrency manual owner: `docs/reference/concurrency/semantics.md`
  - concurrency runtime-plan owner: `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md`

## Immediate Sequence

1. `closure split`
2. `IPO / build-time optimization`

## Parked Corridor

- `phase-96x vm_hako LLVM acceptance cutover`
- monitor-policy decision for the frozen `vm-hako-core` pack remains the only backlog there
