---
Status: Active
Date: 2026-04-24
Scope: current mainline / next lane / parked corridor の one-screen map。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
---

# Workstream Map

## Current Lane

| Item | State |
| --- | --- |
| Current-state SSOT | `docs/development/current/main/CURRENT_STATE.toml` |
| Now | `phase-291x CoreBox surface contract cleanup` |
| Front | read `latest_card_path` in `CURRENT_STATE.toml` |
| Guardrail | `phase-137x observe-only perf reopen rule` |
| Blocker | `phase-291x MapGet/MapHas fusion metadata pending` |
| Next | `same-key MapGet/MapHas fusion metadata preflight` |
| After Next | `lower fusion only after metadata and perf evidence` |

## Current Read

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/phases/phase-291x/README.md`
3. `docs/development/current/main/design/hotline-core-method-contract-ssot.md`
4. `docs/development/current/main/phases/phase-291x/291x-131-hotline-core-method-contract-task-plan.md`
5. `docs/development/current/main/design/current-docs-update-policy-ssot.md`
6. `docs/development/current/main/design/perf-owner-first-optimization-ssot.md`

## Immediate Sequence

1. `bash tools/checks/current_state_pointer_guard.sh`
2. Add same-key MapGet/MapHas fusion metadata before another lowering.
3. Keep compatibility fallback unchanged for rows without carrier metadata.
4. Do not add hot inline lowering without proof/evidence gate.
5. Keep Stage-B adapter thinning as a separate BoxShape series.

## Parked Corridor

- `phase-96x vm_hako LLVM acceptance cutover`
- monitor-policy decision for the frozen `vm-hako-core` pack remains the only backlog there
