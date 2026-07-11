# 291x-779 Parser Token-Cursor Env Cleanup Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/config/env/parser_flags.rs`
- `src/config/env/catalog.rs`
- `src/parser/expressions.rs`
- `src/parser/common/mod.rs`
- `src/parser/statements/helpers.rs`
- `docs/reference/environment-variables.md`
- `docs/development/current/main/CURRENT_STATE.toml`
- `docs/development/current/main/phases/phase-291x/README.md`

## Why

The parser cursor inventory found `NYASH_PARSER_TOKEN_CURSOR` read directly from
multiple parser modules, while parser env ownership is supposed to live under
`src/config/env/parser_flags.rs`.

## Decision

Do not decide the full legacy parser index vs `TokenCursor` helper API in this
card. Keep behavior unchanged and centralize only the env gate.

The broad `expressions.rs` allowance was probed and still guards the old
expression wrapper chain. Removing it without an ownership decision reintroduces
release lib warnings, so it stays with the cursor helper-surface follow-up.

## Landed

- Added `parser_token_cursor_enabled()`.
- Replaced token-cursor direct env reads in parser expression/common/helper
  entry points.
- Documented `NYASH_PARSER_TOKEN_CURSOR`.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

`src/parser/expressions.rs`, `src/parser/cursor.rs`, `src/parser/common/mod.rs`,
and `src/parser/statements/helpers.rs` still contain expression/cursor helper
allowances. Those belong to a separate cursor helper-surface ownership card.

## Proof

- `rg -n "std::env::var\\(\"NYASH_PARSER_TOKEN_CURSOR\"" src/parser/expressions.rs src/parser/common/mod.rs src/parser/statements/helpers.rs -g '*.rs'`
- `bash tools/checks/current_state_pointer_guard.sh`
- `cargo fmt --check`
- `cargo test --lib --no-run`
- `git diff --check`
