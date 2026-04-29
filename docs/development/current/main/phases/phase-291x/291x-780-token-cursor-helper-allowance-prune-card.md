# 291x-780 TokenCursor Helper Method Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/parser/cursor.rs`
- `docs/development/current/main/CURRENT_STATE.toml`
- `docs/development/current/main/phases/phase-291x/README.md`

## Why

After 291x-779 centralized `NYASH_PARSER_TOKEN_CURSOR`, `TokenCursor` still had
four public helper methods hidden behind local `#[allow(dead_code)]`
attributes. Removing only the attributes proved the methods are still unused,
so the cleaner shape is to remove the unused helpers themselves.

## Decision

Remove only the unused `TokenCursor` public helper methods. Keep the broader
expression/common/statement helper ownership question for a separate card.

## Landed

- Removed unused `TokenCursor::peek`.
- Removed unused `TokenCursor::peek_nth`.
- Removed unused `TokenCursor::get_mode`.
- Removed unused `TokenCursor::set_mode`.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

The parser follow-up is now narrower: `src/parser/expressions.rs`,
`src/parser/common/mod.rs`, and `src/parser/statements/helpers.rs` still need a
separate expression/common helper ownership inventory.

## Proof

- `rg -n "peek\\(|peek_nth\\(|get_mode\\(|set_mode\\(|allow\\(dead_code\\)" src/parser/cursor.rs src/parser/expr_cursor.rs -g '*.rs'`
- `bash tools/checks/current_state_pointer_guard.sh`
- `cargo fmt --check`
- `cargo test --lib --no-run`
- `git diff --check`
