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
| Next | land exact `usize` semantics slices from the phase-294x taskboard |
| After Next | resume mimalloc `.hako` rows after hako_alloc non-negative field migration is safe |

## Current Read

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/phases/phase-294x/README.md`
3. `docs/development/current/main/design/usize-semantic-foundation-ssot.md`
4. `docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md`
5. `docs/development/current/main/design/current-docs-update-policy-ssot.md`
6. `docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md`

## Immediate Sequence

1. `bash tools/checks/current_state_pointer_guard.sh`
2. Read `latest_card_path`, `latest_card_summary`, and `current_blocker_token`
   from `CURRENT_STATE.toml`.
3. Start with declared numeric metadata preservation before runtime behavior.
4. Keep runtime semantics, backend lowering, and hako_alloc migration separate.
5. Do not let exact `usize` silently fall back to `Integer(i64)`.

## Parked Corridor

- `phase-96x vm_hako LLVM acceptance cutover`
- monitor-policy decision for the frozen `vm-hako-core` pack remains the only backlog there
