# 291x-729 JoinIR Env Flag Helper Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/config/env/joinir_dev.rs`
- `src/mir/join_ir/mod.rs`
- `src/mir/join_ir/lowering/common.rs`
- `src/mir/join_ir/lowering/loop_to_join/core.rs`
- `src/mir/join_ir/lowering/loop_view_builder.rs`
- `src/mir/join_ir/lowering/{skip_ws,funcscanner_trim,funcscanner_append_defs,stage1_using_resolver,stageb_body,stageb_funcscanner}.rs`

## Why

JoinIR lowering still read `NYASH_JOINIR_LOWER_GENERIC` and
`NYASH_JOINIR_LOWER_FROM_MIR` through the historical
`join_ir::env_flag_is_1(name)` string dispatcher. The SSOT already lives in
`config::env::joinir_dev`, so the dispatcher was an unnecessary second entrance.

## Decision

Read JoinIR dev flags directly through `config::env::joinir_dev::*` helpers.
Remove the string-dispatch helper from `join_ir/mod.rs`.

## Changes

- Replaced `env_flag_is_1("NYASH_JOINIR_LOWER_GENERIC")` with
  `joinir_dev::lower_generic_enabled()`.
- Replaced `env_flag_is_1("NYASH_JOINIR_LOWER_FROM_MIR")` with
  `joinir_dev::lower_from_mir_enabled()`.
- Removed the now-unused `join_ir::env_flag_is_1()` dispatcher.
- Removed the stale occurrence-count comment from the env SSOT.

## Proof

- `cargo fmt --check`
- `bash tools/checks/current_state_pointer_guard.sh`
- `rg -n "env_flag_is_1" src/mir/join_ir src/config -g '*.rs'`
- `rg -n "NYASH_JOINIR_LOWER_GENERIC\"\\)" src/mir/join_ir -g '*.rs'`
- `cargo test --lib --no-run`
- `cargo build --release --bin hakorune`
- `tools/checks/dev_gate.sh quick`
