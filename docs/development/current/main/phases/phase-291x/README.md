---
Status: Active
Date: 2026-04-30
Scope: CoreBox surface catalog / CoreMethodContract cleanup phase front.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/design/mir-root-facade-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-smoke-index.md
---

# Phase 291x: CoreBox Surface Contract Cleanup

## Read First

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/phases/phase-291x/291x-smoke-index.md`
3. `docs/development/current/main/design/current-docs-update-policy-ssot.md`
4. `docs/development/current/main/design/hotline-core-method-contract-ssot.md`

Detailed card history is intentionally not duplicated here. Use numbered
`291x-*` card files as the ledger, and use `CURRENT_STATE.toml` for the latest
checkpoint pointer.

## Current Checkpoint

- Latest landed card: read `latest_card_path` in `CURRENT_STATE.toml`.
- Current blocker token: `phase-291x next compiler-cleanliness lane selection pending`.
- Release lib-warning backlog for this lane is zero.
- `cargo test --lib --no-run` is warning-free.
- The JoinIR / bridge / config-env `dead_code` allowance sweep is closed
  through `291x-775`; the orphan semantics eval scaffold is closed in
  `291x-777`.
- Remaining cleanup is no longer a known dead shelf in this slice; it is
  structural vocabulary inventory for the next selected lane.

## Next Lane Candidates

These came from the post-291x-775 read-only worker inventory. Treat each item as
a separate card; do not mix them with the closed JoinIR / bridge / config-env
allowance sweep.

- Parser static-box seam:
  `src/parser/declarations/static_def/mod.rs` has a module-level `dead_code`
  allowance plus direct parser seam env reads. Decide config/env ownership,
  fail-fast default, and whether the broad allowance can be removed or narrowed.
- Parser expression cursor:
  `src/parser/expressions.rs` keeps a broad allowance and the
  `NYASH_PARSER_TOKEN_CURSOR` bridge. Related cursor code lives in
  `src/parser/common/mod.rs` and `src/parser/cursor.rs`. Decide legacy parser
  index vs `TokenCursor` ownership before deleting or gating anything.
- MIR structural vocabulary:
  `src/mir/policies/cond_profile.rs`, `src/mir/hints.rs`,
  `src/mir/phi_query.rs`, LocalSSA finalizer seams, exit-binding seams, and
  route-shape detectors still have explicit hold comments. Reopen only as a
  BoxShape inventory; these are vocabulary holds, not obvious dead shelves.

No-action inventory: moved-stub / traceability docs and `src/ring0/LAYER_GUARD.rs`
metadata are intentional and outside this cleanup lane.

## Cleanup Closeout Through 291x-777

This burst removed or narrowed the active dead-code shelves around:

- JoinIR lowering module and JoinIR VM bridge module broad allowances
- LowerOnly bridge strict fallback semantics
- bridge dispatch redundant gate state
- legacy JoinIR-to-MIR bridge facade test surface
- bridge metadata PHI stub
- PHI metrics env accessor
- AST lowerer stale value surface
- if-in-loop metadata extraction test surface
- loop route error payload ownership
- ExprLowerer/progress verifier stub surface
- final JoinIR shape allowances
- orphan semantics eval scaffold and broad semantics module allowance

The durable result is that the scanned JoinIR / bridge / config-env surface no
longer relies on `#[allow(dead_code)]` to hide known shelves.

## Current Rules

- Keep BoxShape and BoxCount separate.
- Update `CURRENT_STATE.toml` and the active card first.
- Keep current mirrors thin; do not paste landed-card history into
  `10-Now.md`, `05-Restart-Quick-Resume.md`, or this README.
- New cleanup work should become a focused lane with a small card and proof.
- Do not reopen broad `plan/facts` or `lower::planner_compat` ownership work
  without a focused BoxShape card.
- Keep Stage-B adapter thinning separate from CoreMethodContract migration.
- Keep phase-137x observe-only unless app work reopens a real blocker.

## Task Families

- CoreBox surface catalog design:
  `291x-90-corebox-surface-catalog-design-brief.md`
- Surface inventory:
  `291x-92-corebox-surface-inventory-ledger.md`
- Current task-order baseline:
  `291x-488-current-task-order-baseline-refresh-card.md`
- Historical warning backlog baseline:
  `291x-691-warning-backlog-inventory-doc-sync-card.md`
- Latest cleanup checkpoint:
  `CURRENT_STATE.toml` `latest_card_path`
- Smoke selection:
  `291x-smoke-index.md`
- CoreMethodContract / Hotline policy:
  `docs/development/current/main/design/hotline-core-method-contract-ssot.md`
- MIR root facade policy:
  `docs/development/current/main/design/mir-root-facade-contract-ssot.md`

## Proof Bundle

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
cargo test --lib --no-run
```

Run `tools/checks/dev_gate.sh quick` for milestone checks or before reopening a
broader implementation lane.
