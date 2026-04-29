# 291x-769 JoinIR VM Bridge Meta PHI Stub Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir_vm_bridge/meta.rs`
- `CURRENT_STATE.toml`

## Why

`emit_loop_exit_phi_for_if_modified` was a log-only stub protected by
`#[allow(dead_code)]`. Its only caller was a commented-out line inside the
metadata-aware bridge conversion path.

The surrounding docs still suggested the helper generated loop-exit PHI, but
the active behavior only observes metadata.

## Decision

Delete the inactive PHI stub and stale commented hook. Keep
`convert_join_module_to_mir_with_meta` as the active metadata-aware bridge
entry, but document that PHI generation is not performed there.

Future PHI generation should return as an active lowering contract with fixtures
and a live call site.

## Landed

- Removed `emit_loop_exit_phi_for_if_modified`.
- Removed the commented-out call from `convert_join_module_to_mir_with_meta`.
- Updated the function docs/comments to state metadata observation only.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

This closes the remaining JoinIR VM bridge local `#[allow(dead_code)]` item in
`meta.rs`.

## Proof

- `cargo test --lib --no-run`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
