# 291x-731 ReadOnly Body-Local Slot Shelf Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/lowering/common.rs`
- `src/mir/join_ir/lowering/common/README.md`
- `src/mir/join_ir/lowering/common/body_local_slot.rs`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

`ReadOnlyBodyLocalSlotBox` had no live source/tool callers. Its only remaining
uses were its own unit tests, while active loop-break body-local handling now
uses the derived-slot and condition-only emitters. Keeping the public common
module wire made an obsolete extraction box look like supported lowering
surface.

## Decision

Delete the shelf module and remove it from the common-box README. Historical
phase notes can remain as archive context.

## Changes

- Deleted `common/body_local_slot.rs`.
- Removed the `common::body_local_slot` module wire.
- Removed the README entry for the old read-only body-local slot box.
- Advanced `CURRENT_STATE.toml` to 291x-731.

## Proof

- `rg -n "ReadOnlyBodyLocalSlot|ReadOnlyBodyLocalSlotBox|common::body_local_slot|mod body_local_slot" src tests tools -g '*.rs' -g '*.sh'`
- `cargo fmt --check`
- `bash tools/checks/current_state_pointer_guard.sh`
- `cargo test --lib --no-run`
- `cargo build --release --bin hakorune`
- `tools/checks/dev_gate.sh quick`
