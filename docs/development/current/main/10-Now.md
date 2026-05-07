---
Status: SSOT
Date: 2026-05-07
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
- active lane: `phase-293x real-app bringup`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- phase status: read `phase_status` in `CURRENT_STATE.toml`
- method anchor: read `method_anchor` in `CURRENT_STATE.toml`
- taskboard: read `taskboard` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- current blocker token: `phase-293x typed object i64 field EXE route: newbox field_get field_set before real-app parity`
- update policy:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`

## Next

- continue `phase-293x` real-app bringup
- BoxTorrent mini, binary-trees, mimalloc-lite, the `hako_alloc` VM-only
  page/free-list port, allocator-stress, BoxTorrent allocator-backed store, and
  JSON stream aggregator are landed
- real-app order: BoxTorrent mini -> binary-trees -> mimalloc-lite ->
  allocator port -> allocator-stress app -> BoxTorrent allocator-backed store ->
  JSON stream aggregator
- run `tools/smokes/v2/run.sh --profile integration --suite real-apps --skip-preflight`
  for the active app suite
- run `tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight`
  for the current EXE blocker probe
- next route: typed object i64-field allocation plus slot `field_set` /
  `field_get` for general user-box `newbox`
- if a real app exposes a compiler expressivity blocker, fix the compiler seam
  structurally instead of adding app-side workaround code
- current mirrors are thinned; update `CURRENT_STATE.toml` and the phase-293x
  card/taskboard first
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
2. `docs/development/current/main/phases/phase-293x/README.md`
3. `docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md`
4. `docs/development/current/main/design/current-docs-update-policy-ssot.md`
5. `docs/development/current/main/design/hotline-core-method-contract-ssot.md`
6. `docs/development/current/main/design/perf-owner-first-optimization-ssot.md`

## Proof Bundle

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
tools/smokes/v2/run.sh --profile integration --suite real-apps --skip-preflight
tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight
```
