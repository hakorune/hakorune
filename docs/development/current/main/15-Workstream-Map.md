---
Status: Active
Date: 2026-04-16
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
| Now | `phase-29bq loop owner seam cleanup landing` |
| Front | `cleanup landing -> optimization (kilo / micro-kilo)` |
| Guardrail | `phase-137x` string corridor / `kilo_micro_substring_views_only` |
| Blocker | `active blocker = none; finish the narrow cleanup landing, then stay failure-driven only for the next exact blocker` |
| Next | `return to optimization (kilo / micro-kilo)` |
| After Next | `phase-29bq failure-driven only if a new exact blocker appears` |

## Current Read

  - design owners:
  - implementation lane: `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
  - next layer landing: `docs/development/current/main/design/compiler-expressivity-first-policy.md`
  - roadmap SSOT: `docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md`
  - sibling string guardrail: `docs/development/current/main/phases/phase-137x/README.md`
  - concurrency manual owner: `docs/reference/concurrency/semantics.md`
  - concurrency runtime-plan owner: `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md`

## Immediate Sequence

1. `phase-29bq loop owner seam cleanup landing`
2. `return to optimization (kilo / micro-kilo)`

## Parked Corridor

- `phase-96x vm_hako LLVM acceptance cutover`
- monitor-policy decision for the frozen `vm-hako-core` pack remains the only backlog there
