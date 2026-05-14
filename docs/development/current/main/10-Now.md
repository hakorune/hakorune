---
Status: SSOT
Date: 2026-05-14
Scope: current lane / blocker / next pointer only.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
---

# Self Current Task - Now (main)

## Current

- current-state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- active lane: `phase-293x language minimal surface lane`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- phase status: read `phase_status` in `CURRENT_STATE.toml`
- method anchor: read `method_anchor` in `CURRENT_STATE.toml`
- taskboard: read `taskboard` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- task breakdown:
  `docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md`
- record / packed ArrayBox SSOT:
  `docs/development/current/main/design/record-and-packed-array-lowering-ssot.md`
- mimalloc port purpose:
  `docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md`
- current blocker token: `TRANS-001 transition metadata capsule`
- update policy:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`

## Next

- continue phase-293x after CONTRACT-002; next blocker is TRANS-001 transition metadata capsule
- keep `LOOP-003` open until a JoinIR/CorePlan route is selected; do not source-desugar range loops
- keep allocator-provider activation, hooks, host allocator replacement, and `#[global_allocator]` inactive unless explicitly reopened

## Rules

- keep BoxShape and BoxCount separate
- keep Stage-B adapter thinning separate from CoreMethodContract migration
- do not add hot inline lowering without proof/evidence gate
- do not update current mirrors for every landed card
- update `CURRENT_STATE.toml` and the active card first

## Read Next

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md`
3. `docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md`
4. `docs/development/current/main/phases/phase-293x/README.md`
5. `docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md`
6. `docs/development/current/main/design/current-docs-update-policy-ssot.md`
7. `docs/development/current/main/design/hotline-core-method-contract-ssot.md`
8. `docs/development/current/main/design/perf-owner-first-optimization-ssot.md`

## Proof Bundle

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
tools/smokes/v2/run.sh --profile integration --suite real-apps --skip-preflight
tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight
```
