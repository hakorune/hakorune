# 291x-730 Orphan Simple-While Route Skeleton Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/lowering/simple_while.rs`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

`src/mir/join_ir/lowering/simple_while.rs` was an orphan Phase 188 skeleton:
it was no longer declared from `lowering/mod.rs`, had no live callers, and its
only route detector/lowerer returned `false`/`None`. The active simple-while
path is `simple_while_minimal.rs`, so keeping the orphan file preserved a fake
entry point that could mislead later cleanup.

## Decision

Remove the orphan skeleton file. This does not change route behavior because the
file was not compiled into the module tree.

## Changes

- Deleted `src/mir/join_ir/lowering/simple_while.rs`.
- Advanced `CURRENT_STATE.toml` to 291x-730.

## Proof

- `test ! -e src/mir/join_ir/lowering/simple_while.rs`
- `rg -n "lower_loop_simple_while_route|is_loop_simple_while_route" src tests tools -g '*.rs' -g '*.sh'`
- `cargo fmt --check`
- `bash tools/checks/current_state_pointer_guard.sh`
- `cargo test --lib --no-run`
- `cargo build --release --bin hakorune`
- `tools/checks/dev_gate.sh quick`
