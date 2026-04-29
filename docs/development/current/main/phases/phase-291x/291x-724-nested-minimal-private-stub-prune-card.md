# 291x-724 Nested Minimal Private Stub Prune Card

Status: Landed
Date: 2026-04-29
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/lowering/loop_routes/nested_minimal.rs`

## Why

`validate_nested_structure()` and `has_break_or_continue()` were private TODO stubs in the NestedLoopMinimal lowerer. They were not called by live code, and the only remaining reference was a commented example in the still-stubbed route body.

## Decision

Keep the `nested_minimal` route module and export intact, because current docs and route wiring still reference it. Remove only the disconnected private stubs so the route file no longer carries fake validation vocabulary.

## Changes

- Removed the commented `validate_nested_structure(...)` call from the route implementation sketch.
- Deleted the unused private validation stub and the unused private break/continue helper stub.

## Proof

- `cargo fmt --check`
- `bash tools/checks/current_state_pointer_guard.sh`
- `rg -n "validate_nested_structure|has_break_or_continue" src/mir/join_ir/lowering/loop_routes/nested_minimal.rs`
- `cargo test --lib --no-run`
- `cargo build --release --bin hakorune`
- `tools/checks/dev_gate.sh quick`
