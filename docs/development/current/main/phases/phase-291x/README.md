---
Status: Active
Date: 2026-04-26
Scope: CoreBox surface catalog / CoreMethodContract cleanup phase front.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
  - docs/development/current/main/phases/phase-291x/291x-smoke-index.md
  - docs/development/current/main/phases/phase-291x/291x-255-post-birth-cleanup-task-order-card.md
  - docs/development/current/main/phases/phase-291x/291x-315-joinir-casea-context-label-helper-card.md
  - docs/development/current/main/phases/phase-291x/291x-316-current-pointer-thinning-card.md
  - docs/development/current/main/phases/phase-291x/291x-317-joinir-simple-while-main-gate-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-318-joinir-simple-while-main-gate-helper-card.md
  - docs/development/current/main/phases/phase-291x/291x-319-joinir-casea-update-summary-name-only-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-320-joinir-casea-update-summary-name-only-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-321-joinir-casea-carrier-count-heuristic-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-322-joinir-casea-carrier-count-heuristic-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-323-joinir-loop-update-index-name-heuristic-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-324-joinir-loop-update-rhs-first-classification-card.md
  - docs/development/current/main/phases/phase-291x/291x-325-joinir-loop-update-nested-scope-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-326-joinir-loop-update-nested-scope-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-327-joinir-loop-update-multi-assignment-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-328-joinir-loop-update-all-rhs-classification-card.md
  - docs/development/current/main/phases/phase-291x/291x-329-joinir-loop-update-assignment-value-traversal-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-330-joinir-loop-update-assignment-value-traversal-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-331-joinir-loop-update-summary-helper-split-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-332-joinir-loop-update-summary-helper-split-card.md
  - docs/development/current/main/phases/phase-291x/291x-333-joinir-loop-update-stale-docs-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-334-joinir-loop-update-doc-comment-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-335-joinir-loop-update-caller-surface-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-336-joinir-loop-update-reserved-field-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-337-joinir-loopfeatures-update-summary-surface-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-338-joinir-loopfeatures-update-summary-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-339-joinir-loopfeatures-if-phi-stub-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-340-joinir-loopfeatures-if-phi-stub-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-341-joinir-loopfeatures-nesting-stub-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-342-joinir-loopfeatures-nesting-stub-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-343-joinir-loopfeatures-helper-surface-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-344-joinir-loopfeatures-helper-surface-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-345-joinir-loopfeatures-if-phi-flag-redundancy-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-346-joinir-loopfeatures-if-phi-flag-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-347-joinir-loopfeatures-count-fields-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-348-joinir-loopfeatures-break-continue-count-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-349-joinir-loopfeatures-loopform-constants-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-350-joinir-loopfeatures-loopform-constants-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-351-joinir-loopfeatures-route-surface-review-card.md
  - docs/development/current/main/phases/phase-291x/291x-352-joinir-loopfeatures-route-surface-comment-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-353-joinir-route-detector-legacy-export-inventory-card.md
---

# Phase 291x: CoreBox Surface Contract Cleanup

- Status: active reference lane
- Latest landed cleanup target: read `latest_card_path` in
  `docs/development/current/main/CURRENT_STATE.toml`
- Next implementation target: JoinIR route detector legacy wildcard export prune
- Canonical smoke index:
  `docs/development/current/main/phases/phase-291x/291x-smoke-index.md`
- Sibling guardrail: phase-137x remains observe-only unless app work produces
  a real blocker

## Navigation

Read these first:

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/phases/phase-291x/291x-255-post-birth-cleanup-task-order-card.md`
3. `docs/development/current/main/phases/phase-291x/291x-353-joinir-route-detector-legacy-export-inventory-card.md`
4. `docs/development/current/main/phases/phase-291x/291x-smoke-index.md`
5. `docs/development/current/main/design/hotline-core-method-contract-ssot.md`
6. `docs/development/current/main/design/current-docs-update-policy-ssot.md`

Detailed card history is intentionally not duplicated here. Use the numbered
`291x-*` card files as the ledger.

## Decision

ArrayBox で固定した読み方を CoreBox 全体へ広げる。

```text
surface contract
  -> canonical name / aliases / arity / slot / effect / return

execution dispatch
  -> one invoke seam per Box family

exposure state
  -> runtime / VM / std sugar / docs / smoke pinned state
```

CoreMethodContract cleanup continues as small cards. Do not mix Stage-B adapter
thinning, CoreMethod carrier migration, `.inc` mirror pruning, and hot lowering
in one card.

## Current Rule

- docs-first before code
- current docs update policy:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`
- zero-cost hot boundary and CoreMethodContract migration policy:
  `docs/development/current/main/design/hotline-core-method-contract-ssot.md`
- existing guarded `.inc` mirrors may remain during migration
- new method-name classifier growth needs a contract row, deletion condition,
  and focused guard
- new card work should update `CURRENT_STATE.toml` and the active card first,
  not the restart/current mirrors

## Current Checkpoint

- latest known cleanup checkpoint: `291x-353`
- no-growth baseline: `classifiers=0 rows=0`
- detailed landed history lives in phase card files and the compact
  `landed_tail` in `CURRENT_STATE.toml`
- next cleanup: JoinIR route detector legacy wildcard export prune
- has fallback series: closed and inventoried
- no `.inc` method/box string classifier rows are allowlisted
- metadata-absent direct `MapBox.has` is no longer a supported boundary

## Task Families

| Family | Source |
| --- | --- |
| CoreBox surface catalog design | `291x-90-corebox-surface-catalog-design-brief.md` |
| Surface inventory | `291x-92-corebox-surface-inventory-ledger.md` |
| Post-birth cleanup order | `291x-255-post-birth-cleanup-task-order-card.md` |
| CoreMethodContract / Hotline policy | `docs/development/current/main/design/hotline-core-method-contract-ssot.md` |
| Smoke selection | `291x-smoke-index.md` |

## Proof Bundle

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
tools/checks/dev_gate.sh quick
```

Run `cargo check -q` when Rust code changed. Keep heavy perf ladders for
explicit perf cards or milestone checks.
