---
Status: SSOT
Date: 2026-04-26
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
- current blocker token: `phase-291x JoinIR route detector function-scope compatibility export prune pending`
- update policy:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`

## Next

- verify and prune zero-caller parent `function_scope_capture`
  compatibility export
- cleanup checkpoint: latest known landed card `291x-376`; detailed closed
  history lives in phase card files and the compact `landed_tail` in
  `CURRENT_STATE.toml`
- no-growth checkpoint: `classifiers=0 rows=0`; no `.inc` method/box string
  classifiers are allowlisted
- task-order source:
  `docs/development/current/main/phases/phase-291x/291x-255-post-birth-cleanup-task-order-card.md`
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
