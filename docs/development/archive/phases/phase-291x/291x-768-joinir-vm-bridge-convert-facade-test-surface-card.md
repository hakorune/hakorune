# 291x-768 JoinIR VM Bridge Convert Facade Test-Surface Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir_vm_bridge/convert.rs`
- `src/mir/join_ir_vm_bridge/joinir_function_converter.rs`
- `src/mir/join_ir_vm_bridge/mod.rs`
- `CURRENT_STATE.toml`

## Why

After removing the broad JoinIR VM bridge dead-code allowance, the legacy
`convert_joinir_to_mir` facade still carried `cfg_attr(not(test),
allow(dead_code))`.

Inventory showed the facade is used by bridge-local tests only. Production
conversion now enters through the metadata-aware bridge path, while
`convert_mir_like_inst` remains the active shared helper for block conversion.

## Decision

Move the legacy module conversion facade to explicit `#[cfg(test)]` surface
instead of keeping a production item with a dead-code allowance.

## Landed

- Changed `convert::convert_joinir_to_mir` to `#[cfg(test)]`.
- Changed `JoinIrFunctionConverter::convert_joinir_to_mir` to `#[cfg(test)]`.
- Gated the parent re-export with `#[cfg(test)]` and removed the unused-import
  allowance.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

This closes the JoinIR VM bridge `cfg_attr(not(test), allow(dead_code))` facade
item. The active `convert_mir_like_inst` helper remains production code because
`joinir_block_converter` uses it.

## Proof

- `cargo test --lib --no-run`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
