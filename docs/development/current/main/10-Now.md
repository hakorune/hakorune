---
Status: SSOT
Date: 2026-04-28
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
- active lane: `phase-291x CoreBox surface contract cleanup`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- phase status: read `phase_status` in `CURRENT_STATE.toml`
- method anchor: read `method_anchor` in `CURRENT_STATE.toml`
- taskboard: read `taskboard` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- current blocker token: `phase-291x next compiler-cleanliness lane selection pending`
- update policy:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`

## Next

- choose the next compiler-cleanliness lane, or switch to an explicitly
  reopened non-cleanup blocker
- cleanup checkpoint: latest known card `291x-664`; detailed closed history
  lives in phase card files and `latest_card_path` in `CURRENT_STATE.toml`
- unified-member property cleanup is closed through `291x-655`
- parser member syntax SSOT cleanup is landed through `291x-656`
- planner reject-detail diagnostics cleanup is landed through `291x-657`
- lower planner compat test-only export pruning is landed through `291x-658`
- generic-loop canon reverse export pruning is landed through `291x-659`
- BodyLocalRoute facade pruning is landed through `291x-660`
- DigitPos reject-message test cleanup is landed through `291x-661`
- loop-cond feature pipeline re-export pruning is landed through `291x-662`
- IfPhiJoinFacts alias pruning is landed through `291x-663`
- LoopContinueOnly recipe re-export pruning is landed through `291x-664`
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
2. `docs/development/current/main/phases/phase-291x/README.md`
3. `docs/development/current/main/phases/phase-291x/291x-smoke-index.md`
4. `docs/development/current/main/design/current-docs-update-policy-ssot.md`
5. `docs/development/current/main/design/hotline-core-method-contract-ssot.md`
6. `docs/development/current/main/design/perf-owner-first-optimization-ssot.md`

## Proof Bundle

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
