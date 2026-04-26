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
- current blocker token: `phase-291x JoinIR Case-A context-label helper cleanup pending`
- update policy:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`

## Next

- implement Case-A context-label helper cleanup; minimal target labels should
  consume the descriptor table, while Stage-B labels remain local bridge labels
- cleanup checkpoint: latest known landed card `291x-314`; BuildBox thinning,
  residual MapBox.has sentinel retirement, CoreMethodContract `.inc`
  zero-baseline rebaseline, route-policy owner audit, and route-policy export
  retirement are closed; NeedPolicy owner audit and export retirement are
  closed; SurfacePolicy owner audit and export retirement are closed;
  runtime/meta live table inventory, Using support owner audit/export
  retirement, JsonShapeToMap owner audit/support quarantine, and runtime/meta
  root closeout are closed; post-runtime-meta inventory is closed; JoinIR
  if-target exact allowlist SSOT, prefix policy inventory, prefix helper
  split, type-hint prefix policy inventory, type-hint family table split, and
  GenericTypeResolver P3-C candidate helper audit/retirement, and JoinIR
  residual name-policy inventory, frontend route descriptor table split, and
  Case-A name-policy inventory, Case-A target descriptor table split, Case-A
  fallback dispatch descriptor consumer, and Case-A context-label inventory
  are closed
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
