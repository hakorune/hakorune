# 291x-778 Parser Static-Box Seam Cleanup Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/config/env/parser_flags.rs`
- `src/config/env/catalog.rs`
- `src/parser/declarations/static_def/mod.rs`
- `src/parser/declarations/static_def/members.rs`
- `src/parser/declarations/static_def/validators.rs`
- `docs/reference/environment-variables.md`
- `docs/development/current/main/CURRENT_STATE.toml`
- `docs/development/current/main/phases/phase-291x/README.md`

## Why

The worker inventory found `static_def` with a broad module-level
`dead_code` allowance and parser seam flags read directly through
`std::env::var`. The parser env policy says parser flags should be accessed
through `src/config/env/parser_flags.rs`.

The `validators.rs` file was a no-op shelf with its own dead-code allowances and
no current caller.

## Decision

Keep static-box seam compatibility behavior unchanged, but move the flag reads
behind named config helpers and delete the unused validator shelf.

## Landed

- Added named config helpers for static-box trace, seam, and strict parser
  gates.
- Replaced static-box parser direct env reads with `crate::config::env::*`
  helpers.
- Removed `static_def::validators`.
- Removed the broad `static_def` module dead-code allowance.
- Documented the previously hidden static-box parser flags.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

Parser expression cursor ownership remains a parser cleanup candidate. MIR
structural vocabulary remains a separate BoxShape inventory candidate.

## Proof

- `rg -n "std::env::var|allow\\(dead_code\\)" src/parser/declarations/static_def -g '*.rs'`
- `bash tools/checks/current_state_pointer_guard.sh`
- `cargo test --lib --no-run`
- `git diff --check`
