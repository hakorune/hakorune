# 291x-722 IfMerge Context Shelf Prune Card

Status: Landed
Date: 2026-04-29
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/lowering/if_merge.rs`
- `src/mir/join_ir/lowering/if_lowering_router.rs`
- `src/mir/join_ir/lowering/if_dry_runner.rs`
- `src/mir/builder/if_form.rs`
- `src/mir/join_ir/lowering/if_phi_context.rs`
- `src/tests/mir_joinir_if_select_parts/{if_merge,pattern_matching,type_hints}.rs`

## Why

`IfMergeLowerer.context` was stored but never read, and `IfMergeLowerer::with_debug()` was only a legacy constructor shim. After the IfSelect context shelf prune, the router still accepted a context argument that no lowerer consumed.

## Decision

Keep IfSelect/IfMerge lowering context-free. `IfPhiContext` remains as PHI-side vocabulary for IfPhi/PhiBuilder contracts, not as an If lowering router constructor input.

## Changes

- Removed the unused `IfMergeLowerer.context` field.
- Removed `IfMergeLowerer::with_debug()` and `IfMergeLowerer::with_context()`.
- Removed the `context` argument from `try_lower_if_to_joinir()`.
- Updated builder, dry-runner, tests, and IfPhiContext docs to the context-free router contract.

## Proof

- `cargo fmt --check`
- `bash tools/checks/current_state_pointer_guard.sh`
- `rg -n "IfMergeLowerer::with_context|IfMergeLowerer::with_debug|try_lower_if_to_joinir\\([^\\n]*None|Some\\(&context\\)" src/mir src/tests -g '*.rs'`
- `cargo test --lib --no-run`
- `cargo build --release --bin hakorune`
- `tools/checks/dev_gate.sh quick`
