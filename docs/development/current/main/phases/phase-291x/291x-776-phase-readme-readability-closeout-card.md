# 291x-776 Phase README Readability Closeout Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `docs/development/current/main/phases/phase-291x/README.md`
- `docs/development/current/main/CURRENT_STATE.toml`
- `docs/development/current/main/15-Workstream-Map.md`
- `docs/development/current/main/20-Decisions.md`
- `docs/development/current/main/phases/phase-291x/291x-746-compiler-cleanliness-closeout-inventory-card.md`
- `docs/development/current/main/phases/phase-291x/291x-747-compiler-cleanliness-worker-inventory-card.md`

## Why

After the 291x-775 dead-code allowance sweep, the phase README had become hard
to scan because it duplicated a long historical card list in front matter.
`CURRENT_STATE.toml` also carried a very long one-line status summary.

The current-docs policy says mirrors should stay thin and point to
`CURRENT_STATE.toml` plus card files rather than duplicating landed history.

## Decision

Make the phase README a navigation and closeout summary, not a ledger.
Keep detailed history in numbered card files and the latest pointer in
`CURRENT_STATE.toml`.

## Landed

- Rewrote the phase README front matter to a short Related list.
- Added a readable closeout section for the 291x-775 dead-code allowance sweep.
- Kept task-family navigation focused on durable anchors.
- Shortened `CURRENT_STATE.toml` active lane status while preserving the latest
  pointer and proof status.
- Replaced stale blocker wording in `15-Workstream-Map.md` with
  `CURRENT_STATE.toml` pointer wording.
- Updated the public decisions stub to name `CURRENT_STATE.toml` as the active
  lane/blocker/latest-card SSOT.
- Marked the 291x-746 and 291x-747 inventory queues as historical after
  291x-748 through 291x-775.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

Docs now direct the next agent to choose a new focused compiler-cleanliness lane
instead of re-reading a duplicated historical card list.

## Worker Inventory Result

The closed lane remains closed: no new JoinIR / bridge / config-env dead shelf
was found in the scanned slice. The next cleanup work should be selected as a
new focused lane:

- Parser static-box seam:
  `src/parser/declarations/static_def/mod.rs` has a broad module allowance and
  direct parser seam env reads.
- Parser expression cursor:
  `src/parser/expressions.rs` still has the opt-in `NYASH_PARSER_TOKEN_CURSOR`
  bridge and needs a legacy-index vs cursor ownership decision.
- Semantics scaffold:
  `src/semantics/mod.rs` and `src/semantics/eval.rs` are old PoC skeleton
  surfaces with broad allowances.
- MIR structural vocabulary:
  remaining phase-291x hold comments in MIR are vocabulary / owner seams, not
  quick dead-code deletes.

Do not include moved-stub traceability docs or `src/ring0/LAYER_GUARD.rs` guard
metadata as cleanup debt.

## Proof

- `bash tools/checks/current_state_pointer_guard.sh`
- `cargo test --lib --no-run`
- `git diff --check`
