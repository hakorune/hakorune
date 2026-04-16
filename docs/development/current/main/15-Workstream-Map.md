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
| Now | `phase-29bq selfhost mirbuilder failure-driven` |
| Front | `compiler expressivity first -> selfhost mirbuilder failure-driven` |
| Guardrail | `phase-137x` string corridor / `kilo_micro_substring_views_only` |
| Blocker | `active blocker = none; stay failure-driven and capture the next exact blocker before widening` |
| Next | `compiler expressivity first` |
| After Next | `loop_scan_phi_vars_v0::nested_loop_handoff` cleanup / `loop_cond_shared` owner-surface inventory |

## Current Read

  - design owners:
  - implementation lane: `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
  - next layer landing: `docs/development/current/main/design/compiler-expressivity-first-policy.md`
  - roadmap SSOT: `docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md`
  - sibling string guardrail: `docs/development/current/main/phases/phase-137x/README.md`
  - concurrency manual owner: `docs/reference/concurrency/semantics.md`
  - concurrency runtime-plan owner: `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md`

## Immediate Sequence

1. `compiler expressivity first`
2. `phase-29bq selfhost mirbuilder failure-driven`

## Parked Corridor

- `phase-96x vm_hako LLVM acceptance cutover`
- monitor-policy decision for the frozen `vm-hako-core` pack remains the only backlog there
