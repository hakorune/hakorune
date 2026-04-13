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
| Now | `phase-276x IPO PGO generate/use first cut` |
| Front | `IPO / build-time optimization -> PGO generate/use first cut` |
| Guardrail | `phase-137x` string corridor / `kilo_micro_substring_views_only` |
| Blocker | `ThinLTO and PGO scaffold are closed out; the next queue is PGO generate/use first cut` |
| Next | `IPO / build-time optimization` |
| After Next | `optimization lane closeout judgment` |

## Current Read

  - design owners:
  - implementation lane: `docs/development/current/main/phases/phase-276x/README.md`
  - next layer landing: `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
  - roadmap SSOT: `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
  - sibling string guardrail: `docs/development/current/main/phases/phase-137x/README.md`
  - concurrency manual owner: `docs/reference/concurrency/semantics.md`
  - concurrency runtime-plan owner: `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md`

## Immediate Sequence

1. `IPO / build-time optimization`
2. `optimization lane closeout judgment`

## Parked Corridor

- `phase-96x vm_hako LLVM acceptance cutover`
- monitor-policy decision for the frozen `vm-hako-core` pack remains the only backlog there
