# 293x-302 ASTCLEAN-009 backend/optimizer utility dead_code prune

Status: complete

## Decision

Decision: accepted.

Backend and optimizer utility shelves should not keep anonymous dead-code allowances or stale wrapper modules. Used helpers must not carry `#[allow(dead_code)]`; staged helpers may keep it only with a row rationale.

## Scope

- Remove unused backend error helper wrappers from `error_helpers.rs`.
- Remove obsolete `dead_code` allowances from used backend conversion/error helpers.
- Add `ASTCLEAN-009` rationale comments to retained staged backend utility helpers.
- Delete the stale private `src/mir/optimizer/diagnostics.rs` module; optimizer diagnostics are owned by `src/mir/optimizer_passes/diagnostics.rs`.
- Remove unused optimizer wrapper methods from `MirOptimizer` core.
- Retain `instruction_to_key` with an `ASTCLEAN-009` rationale because optimizer unit tests and diagnostic probes still use it under selective builds.

## Non-goals

- No backend interpreter behavior change.
- No optimizer pass behavior change.
- No public language or MIR semantics change.

## Guard

- `tools/checks/k2_wide_astclean_backend_optimizer_dead_code_guard.sh`

## Local guard

- `bash tools/checks/k2_wide_astclean_backend_optimizer_dead_code_guard.sh` passed locally.
