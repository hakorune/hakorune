# 291x-781 Parser Expression Helper Shelf Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/parser/expressions.rs`
- `src/parser/common/mod.rs`
- `src/parser/statements/helpers.rs`
- `docs/development/current/main/CURRENT_STATE.toml`
- `docs/development/current/main/phases/phase-291x/README.md`

## Why

After 291x-779 and 291x-780, the parser cursor/helper follow-up had two kinds
of leftovers:

- old expression wrapper methods that only forwarded to `expr_*` modules
- unused common/statement helper APIs hidden by local `dead_code` allowances

The active expression chain now calls the `expr_*` helpers directly, so the
wrappers were shelf code rather than vocabulary.

## Decision

Remove only helpers with no current caller. Keep used parser utility methods and
remove their obsolete local allowances.

## Landed

- Removed the unused expression wrapper chain for coalesce/or/and/bit/compare/
  range/term/shift/factor/primary/literal-only.
- Removed unused parser common helpers: `match_any_token`, `is_line_end`, and
  `unknown_span`.
- Removed unused statement helper methods: `err_unexpected` and
  `expect_identifier`.
- Removed obsolete local allowances from used parser utility methods.
- Removed the broad `src/parser/expressions.rs` dead-code allowance.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

The worker-listed parser cleanup candidates are now closed for this pass. The
remaining queue is MIR structural vocabulary / owner-seam inventory.

## Supersession Note

Residual parser sugar/ternary local allowances found by the final broad parser
scan were closed by 291x-782.

## Proof

- `rg -n "allow\\(dead_code\\)" src/parser/expressions.rs src/parser/common/mod.rs src/parser/statements/helpers.rs -g '*.rs'`
- `bash tools/checks/current_state_pointer_guard.sh`
- `cargo fmt --check`
- `cargo test --lib --no-run`
- `git diff --check`
