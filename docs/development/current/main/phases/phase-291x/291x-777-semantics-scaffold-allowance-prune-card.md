# 291x-777 Semantics Scaffold Allowance Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/semantics/mod.rs`
- `src/semantics/eval.rs`
- `src/lib.rs`
- `docs/development/current/main/CURRENT_STATE.toml`
- `docs/development/current/main/phases/phase-291x/README.md`
- `docs/development/current/main/phases/phase-291x/291x-776-phase-readme-readability-closeout-card.md`

## Why

The post-291x-775 worker inventory found `src/semantics` as an old PoC
scaffold with broad `dead_code` allowances. The `eval.rs` file was not wired by
`mod.rs`, and the live `semantics` module exports only the experimental trait
plus the VM-backed adapter.

This is separate from the parser seam/cursor candidates and the MIR structural
vocabulary holds.

## Decision

Keep the public `semantics` module as an explicit experimental facade, but do
not let it hide an orphan evaluator shelf.

- Remove the module-level `dead_code` allowance from `src/semantics/mod.rs`.
- Delete the unwired `src/semantics/eval.rs` scaffold.
- Clarify that production execution remains owned by `backend::mir_interpreter`.

## Landed

- Removed the broad semantics module dead-code allowance.
- Removed the unwired MIR evaluator PoC file.
- Updated the public module comment in `src/lib.rs`.
- Updated the phase README next-lane list so the semantics scaffold is no
  longer listed as pending cleanup.
- Marked the 291x-776 worker inventory semantics candidate as superseded by
  this card.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

The next cleanup choice should be one of:

- parser static-box seam inventory
- parser expression cursor ownership inventory
- MIR structural vocabulary / owner-seam inventory

## Supersession Note

The parser static-box seam candidate was closed by 291x-778. Parser expression
cursor ownership and MIR structural vocabulary remain separate next-lane
candidates.

## Proof

- `rg -n "allow\\(dead_code\\)" src/semantics src/lib.rs -g '*.rs'`
- `bash tools/checks/current_state_pointer_guard.sh`
- `cargo test --lib --no-run`
- `git diff --check`
