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
  - docs/development/current/main/phases/phase-291x/291x-274-docs-smoke-operating-simplification-card.md
  - docs/development/current/main/phases/phase-291x/291x-275-remaining-cleanup-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-276-stageb-buildbox-handoff-adapter-card.md
  - docs/development/current/main/phases/phase-291x/291x-278-stageb-bundle-cli-facade-card.md
  - docs/development/current/main/phases/phase-291x/291x-279-stageb-legacy-boundary-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-280-buildbox-bundle-resolver-split-card.md
  - docs/development/current/main/phases/phase-291x/291x-281-buildbox-remaining-cleanup-order-card.md
  - docs/development/current/main/phases/phase-291x/291x-282-buildbox-parse-source-narrowing-ssot-card.md
  - docs/development/current/main/phases/phase-291x/291x-283-buildbox-bundle-input-collector-split-card.md
  - docs/development/current/main/phases/phase-291x/291x-284-buildbox-fragment-injector-split-card.md
  - docs/development/current/main/phases/phase-291x/291x-285-buildbox-facade-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-286-mir-call-maphas-residual-seam-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-287-mir-call-maphas-sentinel-retirement-card.md
  - docs/development/current/main/phases/phase-291x/291x-288-post-inc-zero-rebaseline-card.md
  - docs/development/current/main/phases/phase-291x/291x-289-mir-call-route-policy-owner-audit-card.md
  - docs/development/current/main/phases/phase-291x/291x-290-mir-call-route-policy-export-retirement-card.md
  - docs/development/current/main/phases/phase-291x/291x-291-mir-call-need-policy-owner-audit-card.md
  - docs/development/current/main/phases/phase-291x/291x-292-mir-call-need-policy-export-retirement-card.md
  - docs/development/current/main/phases/phase-291x/291x-293-mir-call-surface-policy-owner-audit-card.md
  - docs/development/current/main/phases/phase-291x/291x-294-mir-call-surface-policy-export-retirement-card.md
  - docs/development/current/main/phases/phase-291x/291x-295-runtime-meta-live-table-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-296-runtime-meta-using-support-owner-audit-card.md
  - docs/development/current/main/phases/phase-291x/291x-297-runtime-meta-using-support-export-retirement-card.md
  - docs/development/current/main/phases/phase-291x/291x-298-runtime-meta-json-shape-owner-audit-card.md
  - docs/development/current/main/phases/phase-291x/291x-299-runtime-meta-json-shape-support-quarantine-card.md
  - docs/development/current/main/phases/phase-291x/291x-300-runtime-meta-root-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-301-post-runtime-meta-cleanup-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-302-joinir-if-target-exact-allowlist-ssot-card.md
  - docs/development/current/main/phases/phase-291x/291x-303-joinir-if-target-prefix-policy-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-304-joinir-if-target-prefix-helper-split-card.md
  - docs/development/current/main/phases/phase-291x/291x-305-joinir-type-hint-prefix-policy-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-306-joinir-type-hint-family-table-split-card.md
  - docs/development/current/main/phases/phase-291x/291x-307-generic-type-resolver-p3c-candidate-helper-audit-card.md
  - docs/development/current/main/phases/phase-291x/291x-308-generic-type-resolver-p3c-candidate-helper-retirement-card.md
---

# Phase 291x: CoreBox Surface Contract Cleanup

- Status: active reference lane
- Latest landed cleanup target: read `latest_card_path` in
  `docs/development/current/main/CURRENT_STATE.toml`
- Next implementation target: JoinIR residual name-policy inventory
- Canonical smoke index:
  `docs/development/current/main/phases/phase-291x/291x-smoke-index.md`
- Sibling guardrail: phase-137x remains observe-only unless app work produces
  a real blocker

## Navigation

Read these first:

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/phases/phase-291x/291x-255-post-birth-cleanup-task-order-card.md`
3. `docs/development/current/main/phases/phase-291x/291x-308-generic-type-resolver-p3c-candidate-helper-retirement-card.md`
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

- latest known cleanup checkpoint: `291x-308`
- BuildBox thinning series is closed; residual MapBox.has sentinel retirement
  is closed; CoreMethodContract `.inc` classifier baseline is zero;
  `MirCallRoutePolicy`, `MirCallNeedPolicy`, and `MirCallSurfacePolicy`
  exports are retired; runtime/meta live table inventory is closed; Using
  support owner audit/export retirement is closed; JsonShapeToMap owner
  audit/support quarantine is closed; runtime/meta root closeout is closed;
  post-runtime-meta inventory is closed; JoinIR if-target exact allowlist SSOT
  is closed; JoinIR if-target prefix policy inventory/helper split are closed;
  JoinIR type-hint prefix policy inventory is closed; JoinIR type-hint family
  table split is closed; GenericTypeResolver P3-C candidate helper
  audit/retirement is closed; next cleanup is JoinIR residual name-policy
  inventory
- has fallback series: closed and inventoried
- no-growth baseline: `classifiers=0 rows=0`
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
