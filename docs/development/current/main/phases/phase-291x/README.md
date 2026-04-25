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
---

# Phase 291x: CoreBox Surface Contract Cleanup

- Status: active reference lane
- Latest landed cleanup target: read `latest_card_path` in
  `docs/development/current/main/CURRENT_STATE.toml`
- Next implementation target: continue compiler-clean BoxShape cleanup; keep
  the blocked `MapBox.has` fallback baseline closed unless a new owner-path
  change retires it
- Canonical smoke index:
  `docs/development/current/main/phases/phase-291x/291x-smoke-index.md`
- Sibling guardrail: phase-137x remains observe-only unless app work produces
  a real blocker

## Navigation

Read these first:

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/phases/phase-291x/291x-255-post-birth-cleanup-task-order-card.md`
3. `docs/development/current/main/phases/phase-291x/291x-285-buildbox-facade-closeout-card.md`
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

- latest known cleanup checkpoint: `291x-285`
- BuildBox thinning series is closed; next implementation target is cleanup
  selection from the remaining phase-291x inventory
- has fallback series: closed and inventoried
- no-growth baseline: `classifiers=2 rows=2`
- intentional remaining fallback: paired MIR-call `MapBox + has` surface rows
- blocker: revisit only after metadata-absent direct `MapBox.has` is retired
  or replaced by an explicit non-surface contract

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
