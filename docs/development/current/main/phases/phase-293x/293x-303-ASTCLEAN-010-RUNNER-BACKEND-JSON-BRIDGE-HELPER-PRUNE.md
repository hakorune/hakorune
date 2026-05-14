# 293x-303 ASTCLEAN-010 runner backend JSON bridge helper prune

Status: complete

## Decision

Decision: accepted.

Runner JSON bridge helper cleanup should delete unused thin wrappers rather than preserving them behind anonymous `#[allow(dead_code)]`. Active owners remain the scoped/with-vars lowering helpers and current MIR JSON emitters.

## Scope

- Remove unused numeric-core MIR JSON invariant helpers from `mir_json_emit/helpers.rs`.
- Remove unused `emit_extern_call` / `emit_box_call` wrappers from `mir_json_emit/emitters/calls.rs`.
- Remove unused no-vars wrappers and the now-unused `NoVars` scope from JSON v0 expression lowering.
- Remove unused JSON v1 const parser helper and its `ConstValue` import.

## Non-goals

- No MIR JSON schema change.
- No active lowering route change.
- No backend behavior change.

## Guard

- `tools/checks/k2_wide_astclean_runner_json_bridge_helper_guard.sh`

## Local guard

- `bash tools/checks/k2_wide_astclean_runner_json_bridge_helper_guard.sh` passed locally.
