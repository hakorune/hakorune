# 291x-767 JoinIR VM Bridge Module Dead-Code Allow Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir_vm_bridge/mod.rs`
- `src/mir/join_ir_vm_bridge/block_allocator.rs`
- `src/mir/join_ir_vm_bridge/bridge.rs`
- `CURRENT_STATE.toml`

## Why

Removing the module-level `#![allow(dead_code)]` from
`join_ir_vm_bridge/mod.rs` exposed only two local dead shelves:

- `BlockAllocator::next_id_mut`, an old compatibility mutator no active
  converter uses.
- `module_has_joinir_value_ids`, a helper referenced only by a commented-out
  boundary check.

Keeping the broad allow would hide future bridge-local dead code even though
the active warning surface is small.

## Decision

Remove the broad module allowance and delete the two inactive shelves instead
of replacing them with local holds.

Boundary-remap enforcement should return as an active contract with tests and a
live call site, not as a commented-out dormant helper.

## Landed

- Removed `#![allow(dead_code)]` from `join_ir_vm_bridge/mod.rs`.
- Deleted unused `BlockAllocator::next_id_mut`.
- Deleted unused `module_has_joinir_value_ids` and the disabled commented check
  that was its only reference.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

The JoinIR VM bridge module no longer has a broad dead-code allowance. Remaining
bridge holds are local item-level allowances and can be judged independently.

## Proof

- `cargo test --lib --no-run`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
