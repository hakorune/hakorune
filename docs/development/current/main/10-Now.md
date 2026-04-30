---
Status: SSOT
Date: 2026-04-30
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
- active lane: `phase-29ci Program(JSON v0) public compat retirement`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- phase status: read `phase_status` in `CURRENT_STATE.toml`
- method anchor: read `method_anchor` in `CURRENT_STATE.toml`
- taskboard: read `taskboard` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- current blocker token: `phase-29ci raw compat caller migration pending`
- update policy:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`

## Next

- continue `phase-29ci` raw compat caller migration after the P8 helper-local
  legacy delegate retirement slice
- cleanup checkpoint: read `latest_card_path` in `CURRENT_STATE.toml`; detailed
  closed history lives in phase card files
- `--hako-emit-mir-json` stays as the Stage-1 MIR launcher for
  `stage1-env-mir-source`; raw Program(JSON) compat callers are inventoried and
  the next work is migration by caller family
- latest docs/inventory baseline: `291x-691` remains the historical backlog
  inventory; current status is in `CURRENT_STATE.toml`
- do not reopen broad `plan/facts` or `lower::planner_compat` ownership work
  without focused BoxShape lanes and SSOT cards
- normalized-shadow / normalization cleanup burst is closed; larger findings
  move to a new lane
- no-growth checkpoint: `classifiers=0 rows=0`; no `.inc` method/box string
  classifiers are allowlisted
- task-order source:
  `docs/development/current/main/phases/phase-291x/291x-488-current-task-order-baseline-refresh-card.md`
- detailed landed history: phase card files and `CURRENT_STATE.toml`, not this
  mirror

## Rules

- keep BoxShape and BoxCount separate
- keep Stage-B adapter thinning separate from CoreMethodContract migration
- do not add hot inline lowering without proof/evidence gate
- do not update current mirrors for every landed card
- update `CURRENT_STATE.toml` and the active card first

## Read Next

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/phases/archive/phase-29ci/README.md`
3. `docs/development/current/main/phases/phase-29ci/P8-HELPER-LEGACY-DELEGATE-RETIRE.md`
4. `docs/development/current/main/design/current-docs-update-policy-ssot.md`
5. `docs/development/current/main/design/hotline-core-method-contract-ssot.md`
6. `docs/development/current/main/design/perf-owner-first-optimization-ssot.md`

## Proof Bundle

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
