# 291x-738 Exit Meta Builder Shelf Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/lowering/mod.rs`
- `src/mir/join_ir/lowering/exit_meta_builder.rs`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

`IfSumExitMetaBuilderBox` was a thin Phase 118 wrapper around
`carrier_info::ExitMeta::multiple`. It had no production caller; the only live
references were its local unit tests and module export. Keeping it made
`ExitMeta` construction look like it had two authoritative entry points.

## Decision

Delete the unused builder shelf. `carrier_info::ExitMeta` remains the SSOT for
exit metadata vocabulary and construction.

## Changes

- Removed the `exit_meta_builder` module export.
- Deleted `lowering/exit_meta_builder.rs` and its local tests.
- Advanced `CURRENT_STATE.toml` to 291x-738.

## Proof

- `rg -n "exit_meta_builder|IfSumExitMetaBuilderBox|ExitMetaBuilder" src tests -g '*.rs'`
- `cargo fmt --check`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
- `cargo build --release --bin hakorune`
- `tools/checks/dev_gate.sh quick`
