# 291x-742 JoinIR Bridge Target Metadata Truth Sync Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir_vm_bridge_dispatch/targets.rs`
- `src/mir/join_ir_vm_bridge_dispatch/mod.rs`
- `src/mir/join_ir_vm_bridge_dispatch/exec_routes.rs`
- `src/mir/join_ir/lowering/mod.rs`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

The loop `JOINIR_TARGETS` registry still described several non-executing routes
as `Exec` and used `default_enabled` wording that did not match the bridge:
`try_run_joinir_vm_bridge` always requires explicit env enablement, while
Stage1/StageB lower-only routes generate JoinIR and return to VM Route A.

`FuncScannerBox.append_defs/2` also no longer has a direct VM bridge exec route
after the 291x-737 direct lowerer shelf prune; its active loop owner remains the
Case-A lowering target policy.

## Decision

Keep `JOINIR_TARGETS` as the loop-lowering registration SSOT, but make its
metadata truthful:

- `Exec` means a routed JoinIR VM execution path exists.
- `LowerOnly` means lowering/structure ownership only, with normal VM execution.
- Loop VM bridge entry remains env-gated regardless of `default_enabled`.

## Changes

- Marked append-defs and Stage1/StageB loop rows as `LowerOnly`.
- Set loop bridge `default_enabled` metadata to false for the loop table.
- Clarified `default_enabled` semantics for loop targets versus if targets.
- Updated bridge/lowering comments that previously implied kind-driven dispatch
  or env-free loop bridge execution.
- Added a unit test pinning non-exec loop bridge rows as lower-only.
- Advanced `CURRENT_STATE.toml` to 291x-742.

## Proof

- `rg -n "LowerOnly.?→.?Exec|Exec 昇格|default_enabled=true|env フラグなしでも有効|Exec 対象" src/mir/join_ir_vm_bridge_dispatch src/mir/join_ir/lowering/mod.rs -g '*.rs'`
- `cargo fmt --check`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
- `cargo build --release --bin hakorune`
- `tools/checks/dev_gate.sh quick`
