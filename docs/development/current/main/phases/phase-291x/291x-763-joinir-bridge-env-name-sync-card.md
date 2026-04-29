# 291x-763 JoinIR Bridge Env Name Sync Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `join_ir_vm_bridge_dispatch/env_flags.rs`
- `join_ir_vm_bridge_dispatch/README.md`
- `CURRENT_STATE.toml`

## Why

`JoinIrEnvFlags::joinir_experiment` actually stored
`joinir_core_enabled()`. Since JoinIR core is now always on, the old field name
made bridge gating look like it still depended on `NYASH_JOINIR_EXPERIMENT`.

## Decision

Rename the field to `joinir_core` without changing bridge behavior.

`is_bridge_enabled()` still requires core-on plus VM bridge enabled; core-on is
always true through the config SSOT.

## Landed

- Renamed `joinir_experiment` to `joinir_core`.
- Updated dispatch README env wording.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

The bridge env naming drift item is closed. Remaining structural cleanup is now:

- bridge strict/LowerOnly semantics
- broad JoinIR lowering module-level `dead_code` allowance inventory

## Proof

- `rg -n "joinir_experiment" src/mir/join_ir_vm_bridge_dispatch -g '*.rs'`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
