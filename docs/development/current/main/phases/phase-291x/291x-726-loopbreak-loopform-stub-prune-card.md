# 291x-726 LoopBreak LoopForm Stub Prune Card

Status: Landed
Date: 2026-04-29
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/lowering/loop_route_router.rs`
- `src/mir/join_ir/lowering/loop_routes/mod.rs`
- `src/mir/join_ir/lowering/loop_routes/with_break.rs`

## Why

`loop_routes/with_break.rs` was a compatibility stub. The only live caller was
the LoopForm route router, and the function always returned `None`, so the
router already fell through to the existing lowering path.

## Decision

Keep the `LoopRouteKind::LoopBreak` match arm for route-kind exhaustiveness, but
make the fallthrough explicit in the router. Live LoopBreak lowering remains
owned by the plan/composer route path and `loop_with_break_minimal.rs`.

## Changes

- Removed the `with_break` module and its re-export.
- Replaced the router call into the stub with an explicit debug-only fallthrough
  note.
- Removed ignored placeholder tests that referenced the deleted stub API.

## Proof

- `cargo fmt --check`
- `bash tools/checks/current_state_pointer_guard.sh`
- `rg -n "lower_loop_with_break_to_joinir|loop_routes::with_break|pub mod with_break|pub use with_break|with_break\\.rs" src/mir src/tests -g '*.rs'`
- `cargo test --lib --no-run`
- `cargo build --release --bin hakorune`
- `tools/checks/dev_gate.sh quick`
