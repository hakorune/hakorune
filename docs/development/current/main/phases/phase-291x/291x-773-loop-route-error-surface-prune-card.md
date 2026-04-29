# 291x-773 Loop Route Error Surface Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/frontend/ast_lowerer/loop_routes/mod.rs`
- `src/mir/join_ir/frontend/ast_lowerer/loop_frontend_binding.rs`
- `CURRENT_STATE.toml`

## Why

`LoopRouteLowerer` had no implementations or callers, so it was a dead
interface shelf.

`LoweringError` needed a dead-code allowance because its payload fields were
only visible through derived `Debug`, which the compiler does not count as a
real read.

## Decision

Delete the unused trait and make `LoweringError` own a concrete `Display`
contract. The route binding panic now prints the Display form, so error payloads
are actively read.

## Landed

- Removed `LoopRouteLowerer`.
- Removed the unused `JsonParseError` variant.
- Added `Display` and `Error` impls for `LoweringError`.
- Changed `LoopFrontendBinding` panic formatting from `Debug` to `Display`.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

This closes both loop-routes module `#[allow(dead_code)]` items.

## Proof

- `cargo test --lib --no-run`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
