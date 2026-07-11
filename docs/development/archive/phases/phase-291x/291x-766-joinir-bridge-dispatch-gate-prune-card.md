# 291x-766 JoinIR Bridge Dispatch Gate Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir_vm_bridge_dispatch/env_flags.rs`
- `src/mir/join_ir_vm_bridge_dispatch/exec_routes.rs`
- `src/mir/join_ir_vm_bridge_dispatch/README.md`
- `CURRENT_STATE.toml`

## Why

Worker inventory found two small duplicate truths in JoinIR VM bridge dispatch:

- `JoinIrEnvFlags` stored `joinir_core` even though
  `joinir_vm_bridge_enabled()` already owns the core always-on/deprecation
  policy.
- Exec routes had separate strict/non-strict success branches with identical
  output and exit behavior.

Both made the dispatch surface look wider than its behavior.

## Decision

Keep bridge enablement policy in `src/config/env/joinir_flags.rs`.
`join_ir_vm_bridge_dispatch/env_flags.rs` now only wraps the resulting VM bridge
enabled bit.

Exec route strict handling remains at the caller/failure boundary. On success,
strict and non-strict routes both emit the same result and exit with the same
code, so the success branch is shared.

## Landed

- Removed `JoinIrEnvFlags::joinir_core`.
- Made `JoinIrEnvFlags::is_bridge_enabled()` return the bridge-enabled bit
  directly.
- Removed duplicate strict success branches from `try_run_skip_ws` and
  `try_run_trim`.
- Synced dispatch README wording to distinguish loop bridge env opt-in from If
  table defaults.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

This closes the bridge env double-gate and exec strict-success duplicate items.
`JOINIR_TARGETS` remains a loop-lowering registration SSOT; the
`FuncScannerBox.append_defs/2` LowerOnly row is intentionally retained because
`is_loop_lowered_function()` depends on the full loop target table.

## Proof

- `cargo test --lib --no-run`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
