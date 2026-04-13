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
| Front | `semantic simplification bundle closed out -> next design lane is memory-effect layer` |
| Guardrail | `phase-137x` string corridor / `kilo_micro_substring_views_only` |
| Blocker | `memory-effect layer is active; M0/M1/M2 are landed, and the next queue starts at M3` |
| Next | `memory-effect layer` |
| After Next | `escape / barrier -> LLVM attrs` |

## Current Read

- design owners:
  - implementation lane: `docs/development/current/main/phases/phase-163x/README.md`
  - next layer landing: `docs/development/current/main/phases/phase-260x/README.md`
  - roadmap SSOT: `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
  - sibling string guardrail: `docs/development/current/main/phases/phase-137x/README.md`
  - concurrency manual owner: `docs/reference/concurrency/semantics.md`
  - concurrency runtime-plan owner: `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md`

## Immediate Sequence

1. `memory-effect layer`
2. `escape / barrier -> LLVM attrs`
3. `numeric loop / SIMD`

## Parked Corridor

- `phase-96x vm_hako LLVM acceptance cutover`
- monitor-policy decision for the frozen `vm-hako-core` pack remains the only backlog there
