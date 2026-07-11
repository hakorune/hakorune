# 291x-741 JoinIR Exec Generic Route Shelf Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir_vm_bridge_dispatch/exec_routes.rs`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

`run_generic_joinir_route` was a Phase 82 compatibility shelf guarded with
`#[allow(dead_code)]`. The live JoinIR VM bridge exec routes are the explicit
`try_run_skip_ws` and `try_run_trim` paths, so keeping a generic helper with no
caller added dead structure to the dispatch layer.

## Decision

Delete the unused helper and keep the explicit exec routes as the only active
execution surface.

## Changes

- Removed `run_generic_joinir_route`.
- Removed the local `#[allow(dead_code)]` shelf.
- Advanced `CURRENT_STATE.toml` to 291x-741.

## Proof

- `rg -n "run_generic_joinir_route|Exec routes 統一ヘルパー" src -g '*.rs'`
- `cargo fmt --check`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
- `cargo build --release --bin hakorune`
- `tools/checks/dev_gate.sh quick`
