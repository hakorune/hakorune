# 291x-791 MIR Hints Scaffold Retire Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/mod.rs`
- `src/mir/hints.rs`
- `docs/guides/scope-hints.md`
- `docs/guides/scopebox.md`
- `docs/guides/testing-matrix.md`
- `docs/reference/invariants.md`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

The last remaining broad MIR cleanup hold was `src/mir/hints.rs`, but repository
evidence showed the real builder hint path already lives elsewhere:

- `MirBuilder` routes scope/join hint calls through
  `hakorune_mir_builder::MetadataContext`
- `MetadataContext` owns the actual no-op hint sink
- `src/mir/hints.rs` had no runtime callers and only carried a stale standalone
  env/doc contract (`NYASH_MIR_HINTS`, `NYASH_MIR_TRACE_HINTS`)

That made `src/mir/hints.rs` an obsolete scaffold rather than an active owner.

## Decision

Retire the standalone MIR hints scaffold and update current docs to describe the
MetadataContext-owned no-op hint path instead.

## Landed

- Removed `src/mir/hints.rs` and its `pub mod hints` export.
- Rewrote scope-hints/scopebox docs so they no longer advertise the retired
  env-based MIR hint contract.
- Removed the stale MIR hints row from the testing matrix and the outdated
  observability line from `docs/reference/invariants.md`.
- Advanced current mirrors and `CURRENT_STATE.toml` to a post-cleanup lane
  selection checkpoint.

## Remaining Queue Impact

The audited MIR structural dead-shelf set is now closed without a remaining
broad dead-code hold. Follow-up work should open a new owner-backed lane rather
than reviving the retired standalone hints scaffold.

## Proof

- `rg -n "NYASH_MIR_HINTS|NYASH_MIR_TRACE_HINTS|src/mir/hints.rs" src docs`
- `bash tools/checks/current_state_pointer_guard.sh`
- `cargo fmt --check`
- `cargo test --lib --no-run`
- `git diff --check`
