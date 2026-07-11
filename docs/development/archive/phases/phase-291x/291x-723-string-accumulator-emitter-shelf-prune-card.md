# 291x-723 String Accumulator Emitter Shelf Prune Card

Status: Landed
Date: 2026-04-29
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/lowering/common.rs`
- `src/mir/join_ir/lowering/common/string_accumulator_emitter.rs`

## Why

`emit_string_concat()` was an isolated JoinIR helper with no live caller. Its only references were the helper's own unit test and the `common.rs` module declaration, so it no longer represented an active lowering route.

## Decision

Remove the dead shelf instead of keeping a standalone emitter vocabulary. String concatenation remains represented through the active lowering routes that actually emit `BinOp(Add)`.

## Changes

- Removed the `string_accumulator_emitter` module declaration.
- Deleted the unused emitter helper and its self-contained unit test.

## Proof

- `cargo fmt --check`
- `bash tools/checks/current_state_pointer_guard.sh`
- `rg -n "emit_string_concat|string_accumulator_emitter" src/mir src/tests -g '*.rs'`
- `cargo test --lib --no-run`
- `cargo build --release --bin hakorune`
- `tools/checks/dev_gate.sh quick`
