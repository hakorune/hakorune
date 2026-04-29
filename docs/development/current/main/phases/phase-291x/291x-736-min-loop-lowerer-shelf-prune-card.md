# 291x-736 Min Loop Lowerer Shelf Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/lowering/mod.rs`
- `src/mir/join_ir/lowering/min_loop.rs`
- `src/mir/join_ir/mod.rs`
- `src/tests/mir_joinir_min.rs`
- `src/mir/join_ir/README.md`
- `docs/development/current/main/phases/phase-291x/291x-719-value-id-ranges-min-loop-shelf-prune-card.md`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

`lower_min_loop_to_joinir` was a fixed Phase 26-H demonstration lowerer for
`JoinIrMin.main/0`. It had no production caller; the remaining crate references
were a public compatibility re-export and an ignored auto-lowering experiment
test. Keeping it after the min-loop ValueId range vocabulary was already removed
made an inactive route look supported.

## Decision

Delete the fixed min-loop lowerer shelf and its public re-export. Keep the
JoinIR type sanity/manual construction tests that still exercise the core IR
types without depending on the retired lowerer.

## Changes

- Removed the `min_loop` module and `lower_min_loop_to_joinir` re-exports.
- Deleted `lowering/min_loop.rs`.
- Removed the ignored min-loop auto-lowering test that called the deleted
  lowerer.
- Updated JoinIR lowering README and the earlier 291x-719 min-loop range card.
- Advanced `CURRENT_STATE.toml` to 291x-736.

## Proof

- `rg -n "lower_min_loop_to_joinir|pub mod min_loop|pub use min_loop" src tests tools -g '*.rs' -g '*.sh'`
- `cargo fmt --check`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
- `cargo build --release --bin hakorune`
