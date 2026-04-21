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
| Now | `phase-291x CoreBox surface catalog` |
| Front | `ArrayBox precedent landed -> StringBox catalog landed -> MapBox catalog landed` |
| Guardrail | `phase-137x observe-only perf reopen rule` |
| Blocker | `CoreBox legacy std/debt surfaces need cleanup triage after first catalog slices` |
| Next | `.hako MapBox extended-route cleanup decision` |
| After Next | `namespace/static box/alias resolution SSOT` |

## Current Read

  - design owners:
  - implementation lane: `docs/development/current/main/phases/phase-291x/README.md`
  - phase brief: `docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md`
  - taskboard: `docs/development/current/main/phases/phase-291x/291x-95-mapbox-hako-extended-route-cleanup-card.md`
  - inventory: `docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md`
  - sibling string guardrail: `docs/development/current/main/phases/phase-137x/README.md`

## Immediate Sequence

1. `phase-291x docs-first CoreBox surface inventory`
2. `phase-291x StringBox catalog/invoke seam` landed
3. `phase-291x stable StringBox smoke` landed
4. `phase-291x MapBox first catalog slice` landed
5. `phase-291x safe legacy cleanup deletions` landed
6. `phase-291x apps.lib.boxes.map_std prelude cleanup card` landed
7. `phase-291x .hako MapBox extended-route cleanup decision`

## Parked Corridor

- `phase-96x vm_hako LLVM acceptance cutover`
- monitor-policy decision for the frozen `vm-hako-core` pack remains the only backlog there
