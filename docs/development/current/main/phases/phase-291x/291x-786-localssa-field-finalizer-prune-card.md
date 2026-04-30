# 291x-786 LocalSSA Field Finalizer Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/builder/ssa/local.rs`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

Worker inventory on the remaining MIR seam queue showed one LocalSSA hold with
no live owner path:

- `finalize_field_base_and_args`

Repository search found no `src/` or `tests/` callers. The helper had become a
pure staged seam kept only by a local `#[allow(dead_code)]`.

## Decision

Delete only the zero-use `finalize_field_base_and_args` helper. Keep the rest of
`ssa/local.rs` intact.

## Landed

- Removed `finalize_field_base_and_args` from `src/mir/builder/ssa/local.rs`.

## Remaining Queue Impact

The LocalSSA residue queue no longer includes the field-use finalizer seam. Any
future LocalSSA work must reopen with a real caller or a fresh owner-path card.

## Proof

- `rg -n "finalize_field_base_and_args" src tests -g '*.rs'`
- `bash tools/checks/current_state_pointer_guard.sh`
- `cargo fmt --check`
- `cargo test --lib --no-run`
- `git diff --check`
