# 291x-725 IfPhi LoopForm Stub Prune Card

Status: Landed
Date: 2026-04-29
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/lowering/loop_route_router.rs`
- `src/mir/join_ir/lowering/loop_routes/mod.rs`
- `src/mir/join_ir/lowering/loop_routes/with_if_phi.rs`

## Why

`loop_routes/with_if_phi.rs` was a compatibility stub. The only live caller was the LoopForm route router, and the function always returned `None`, so the router already fell through to existing lowering.

## Decision

Keep the `LoopRouteKind::IfPhiJoin` match arm for route-kind exhaustiveness, but make the fallthrough explicit in the router. Live IfPhiJoin lowering remains owned by the plan/AST route path and `loop_with_if_phi_if_sum.rs`.

## Changes

- Removed the `with_if_phi` module and its re-export.
- Replaced the router call into the stub with an explicit debug-only fallthrough note.
- Removed ignored placeholder tests that referenced the deleted stub API.

## Proof

- `cargo fmt --check`
- `bash tools/checks/current_state_pointer_guard.sh`
- `rg -n "lower_loop_with_conditional_phi_to_joinir|loop_routes::with_if_phi|pub mod with_if_phi|pub use with_if_phi" src/mir src/tests -g '*.rs'`
- `cargo test --lib --no-run`
- `cargo build --release --bin hakorune`
- `tools/checks/dev_gate.sh quick`
