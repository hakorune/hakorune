---
Status: Active
Date: 2026-04-30
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
| Now | read `active_lane` in `CURRENT_STATE.toml` |
| Front | read `latest_card_path` in `CURRENT_STATE.toml` |
| Guardrail | `phase-137x observe-only perf reopen rule` |
| Blocker | read `current_blocker_token` in `CURRENT_STATE.toml` |
| Next | retire or archive the next Program(JSON v0) keeper bucket from the latest card |
| After Next | keep Stage-B adapter thinning and native storage lanes separate unless reopened with evidence |

## Current Read

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/phases/phase-29cv/README.md`
3. `docs/development/current/main/phases/phase-29cv/P0-POST-EXE-DIRECT-KEEPER-INVENTORY.md`
4. `docs/development/current/main/design/hotline-core-method-contract-ssot.md`
5. `docs/development/current/main/design/current-docs-update-policy-ssot.md`
6. `docs/development/current/main/design/perf-owner-first-optimization-ssot.md`

## Immediate Sequence

1. `bash tools/checks/current_state_pointer_guard.sh`
2. Read `latest_card_path`, `latest_card_summary`, and `current_blocker_token`
   from `CURRENT_STATE.toml`.
3. If compiler-cleanliness continues, open one focused card for the selected
   keeper bucket before editing code.
4. Keep Stage-B adapter thinning as a separate BoxShape series.
5. Do not add hot inline lowering without proof/evidence gate.

## Parked Corridor

- `phase-96x vm_hako LLVM acceptance cutover`
- monitor-policy decision for the frozen `vm-hako-core` pack remains the only backlog there
