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
| Now | `phase-292x .inc codegen thin tag cleanup` |
| Front | `array_rmw_window route tag landed -> array_string_len_window len-only route tag landed` |
| Guardrail | `phase-137x observe-only perf reopen rule` |
| Blocker | `array_string_len_window source-reuse modes must move to MIR-owned route metadata before C analyzer deletion` |
| Next | `array_string_len_window source-reuse modes` |
| After Next | `generic method route policy metadata` |

## Current Read

  - design owners:
  - implementation lane: `docs/development/current/main/phases/phase-292x/README.md`
  - phase brief: `docs/development/current/main/phases/phase-292x/292x-90-inc-codegen-thin-tag-design-brief.md`
  - taskboard: `docs/development/current/main/phases/phase-292x/292x-94-array-string-len-window-route-card.md`
  - inventory: `docs/development/current/main/phases/phase-292x/292x-92-inc-codegen-analysis-debt-ledger.md`
  - sibling string guardrail: `docs/development/current/main/phases/phase-137x/README.md`

## Immediate Sequence

1. `phase-291x CoreBox surface catalog` landed
2. `phase-292x docs-first .inc thin tag phase cut`
3. `phase-292x array_rmw_window MIR-owned route tag`
4. `phase-292x array_string_len_window source-reuse metadata routes`
5. `phase-292x generic method route policy metadata`
6. `phase-292x string concat / direct-set windows metadata-only`
7. `phase-292x exact seed ladders to function-level backend route tags`

## Parked Corridor

- `phase-96x vm_hako LLVM acceptance cutover`
- monitor-policy decision for the frozen `vm-hako-core` pack remains the only backlog there
