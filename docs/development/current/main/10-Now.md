---
Status: SSOT
Date: 2026-04-25
Scope: current lane / blocker / next pointer only.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
---

# Self Current Task — Now (main)

## Current

- current-state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- active lane: `phase-291x CoreBox surface contract cleanup`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- phase status: read `phase_status` in `CURRENT_STATE.toml`
- method anchor: read `method_anchor` in `CURRENT_STATE.toml`
- taskboard: read `taskboard` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- current blocker token: `phase-291x push CoreMethod route carrier implementation pending`
- update policy:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`

## Next

- implement the `push` CoreMethod route carrier after the mutating boundary preflight
- keep legacy `push` mirror rows until metadata-absent mutating boundary coverage exists
- keep BoxShape and BoxCount separate
- keep Stage-B adapter thinning separate from CoreMethodContract migration
- do not add hot inline lowering without proof/evidence gate
- do not update current mirrors for every landed card
- update `CURRENT_STATE.toml` and the active card first

## Read Next

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/phases/phase-291x/README.md`
3. `docs/development/current/main/design/current-docs-update-policy-ssot.md`
4. `docs/development/current/main/design/hotline-core-method-contract-ssot.md`
5. `docs/development/current/main/design/perf-owner-first-optimization-ssot.md`

## Proof Bundle

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
