# 291x-765 JoinIR Lowering Module Dead-Code Allow Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/lowering/mod.rs`
- `CURRENT_STATE.toml`

## Why

After the JoinIR lowering cleanup burst, the broad module-level
`#![allow(dead_code)]` in `lowering/mod.rs` no longer masked any release lib
warnings. Keeping it would hide future dead shelves in a high-traffic lowering
namespace.

## Decision

Remove the module-level allowance. Future holds must be local and documented at
the exact item that needs them.

## Landed

- Removed `#![allow(dead_code)]` from `src/mir/join_ir/lowering/mod.rs`.
- Verified `cargo test --lib --no-run` remains warning-free.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

The broad JoinIR lowering module dead-code allowance item is closed. Remaining
cleanup is now mostly local/documentation inventory:

- older historical phase cards mention now-closed surfaces as history
- non-lowering JoinIR/bridge modules still have their own local holds

## Proof

- `cargo test --lib --no-run`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
